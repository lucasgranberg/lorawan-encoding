use bitfield_struct::bitfield;
use zerocopy::{
    big_endian::U16, FromBytes, Immutable, IntoBytes, KnownLayout, TryFromBytes, Unaligned,
};

use crate::Error;

#[bitfield(u8)]
#[derive(FromBytes, IntoBytes, KnownLayout, Immutable, Unaligned)]
pub struct FCtrl {
    adr: bool,
    _rfu: bool,
    ack: bool,
    f_pending: bool,
    #[bits(4)]
    f_opts_len: usize,
}

#[derive(FromBytes, IntoBytes, KnownLayout, Immutable, Unaligned)]
#[repr(C)]
pub struct FHDR {
    dev_addr: [u8; 4],
    f_ctrl: FCtrl,
    f_cnt: U16,
}
impl FHDR {
    pub fn new(dev_addr: [u8; 4], f_ctrl: FCtrl, f_cnt: u16) -> Self {
        Self {
            dev_addr,
            f_ctrl,
            f_cnt: f_cnt.into(),
        }
    }
}

#[derive(TryFromBytes, KnownLayout, Unaligned)]
#[repr(u8)]
pub enum DownlinkHeader {
    Unconfirmed = 0b01100000,
    Confirmed = 0b10100000,
}

#[derive(TryFromBytes)]
pub struct Downlink {
    mhdr: DownlinkHeader,
    fhdr: FHDR,
    data: [u8],
}
#[derive(IntoBytes, KnownLayout, Unaligned, Immutable)]
#[repr(u8)]
pub enum UplinkHeader {
    Unconfirmed = 0b01000000,
    Confirmed = 0b10000000,
}
#[derive(IntoBytes, KnownLayout, Unaligned, Immutable)]
#[repr(C)]
pub struct Uplink {
    mhdr: UplinkHeader,
    f_hdr: FHDR,
}
impl Uplink {
    pub fn encode<'a>(
        buf: &'a mut [u8],
        confirmed: bool,
        dev_addr: [u8; 4],
        adr: bool,
        ack: bool,
        f_port: u8,
        f_cnt: u16,
        f_opts: &[u8],
        payload: &[u8],
    ) -> Result<&'a [u8], Error> {
        let f_opts_len = f_opts.len();
        let payload_len = payload.len();
        let len = size_of::<FHDR>() + f_opts_len + 1 + payload_len;
        if buf.len() < len {
            return Err(Error::Size);
        }
        let f_ctrl = FCtrl::default()
            .with_ack(ack)
            .with_adr(adr)
            .with_f_opts_len(f_opts.len());
        let mhdr = if confirmed {
            UplinkHeader::Confirmed
        } else {
            UplinkHeader::Unconfirmed
        };
        let f_hdr = FHDR::new(dev_addr, f_ctrl, f_cnt);
        let uplink = Self { mhdr, f_hdr };
        let mut pos = size_of::<Self>();
        uplink.write_to(&mut buf[0..pos]).map_err(|_| Error::Size)?;
        buf[pos..pos + f_opts_len].copy_from_slice(f_opts);
        if payload_len > 0 {
            pos += f_opts_len;
            buf[pos] = f_port;
            buf[pos..pos + payload_len].copy_from_slice(payload);
        }
        Ok(&buf[..len])
    }
    pub fn to_bytes(&self) {}
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn encode_uplink() {
        let mut buf = [0u8; 255];
        let payload: [u8; 2] = [0x01, 0x02];
        let uplink = Uplink::encode(
            &mut buf,
            true,
            [0, 1, 2, 3],
            true,
            true,
            4,
            5,
            &[],
            &payload,
        )
        .unwrap();
        assert_eq!(uplink, &[128, 0, 1, 2, 3, 5, 0, 5, 1, 2])
    }
    #[test]
    fn encode_empty_uplink() {
        let mut buf = [0u8; 255];
        let uplink =
            Uplink::encode(&mut buf, true, [0, 1, 2, 3], true, true, 4, 5, &[], &[]).unwrap();
        assert_eq!(uplink, &[128, 0, 1, 2, 3, 5, 0, 5])
    }
}
