use bitfield_struct::bitfield;
use zerocopy::{
    little_endian::U16, FromBytes, Immutable, IntoBytes, KnownLayout, TryFromBytes, Unaligned,
};

use crate::{
    crypto::{Crypto, Encrypter as _, Key, Mac as _},
    mac_command::uplink::{encode_maccommands, UplinkMacCommmand},
    types::{DevAddr, MIC},
    Error,
};
pub mod downlink;
pub mod uplink;

#[bitfield(u8)]
#[derive(FromBytes, IntoBytes, KnownLayout, Immutable, Unaligned)]
pub struct FCtrl {
    #[bits(4)]
    f_opts_len: usize,
    f_pending: bool,
    ack: bool,
    _rfu: bool,
    adr: bool,
}

#[derive(FromBytes, IntoBytes, Immutable, Unaligned)]
#[repr(packed)]
pub struct FHDR {
    dev_addr: DevAddr,
    f_ctrl: FCtrl,
    f_cnt: U16,
    f_opts: [u8; 15],
}
impl FHDR {
    pub fn new(
        dev_addr: DevAddr,
        adr: bool,
        ack: bool,
        f_pending: bool,
        f_cnt: u16,
        f_opts: &[u8],
    ) -> Result<Self, Error> {
        let mut f_opts_buf = [0u8; 15];
        if f_opts.len() > f_opts_buf.len() {
            return Err(Error::Size);
        }
        f_opts_buf[0..f_opts.len()].copy_from_slice(f_opts);
        Ok(Self {
            dev_addr,
            f_ctrl: FCtrl::new()
                .with_adr(adr)
                .with_ack(ack)
                .with_f_pending(f_pending)
                .with_f_opts_len(f_opts.len()),
            f_cnt: f_cnt.into(),
            f_opts: f_opts_buf,
        })
    }
    pub fn dev_addr(&self) -> DevAddr {
        self.dev_addr
    }
    pub fn f_ctrl(&self) -> FCtrl {
        self.f_ctrl
    }
    pub fn f_cnt(&self) -> u16 {
        self.f_cnt.get()
    }
    pub fn f_opts(&self) -> &[u8] {
        &self.f_opts[..self.f_ctrl.f_opts_len()]
    }
}

#[derive(KnownLayout, FromBytes, IntoBytes, Immutable, Unaligned)]
#[repr(C, packed)]
pub struct FRMPayload {
    pub f_port: u8,
    pub data: [u8],
}
impl FRMPayload {
    pub fn new_from_slice<'a>(buf: &'a mut [u8], f_port: u8, slice: &[u8]) -> &'a mut Self {
        let payload = Self::mut_from_bytes(&mut buf[..1 + slice.len()]).unwrap();
        payload.f_port = f_port;
        payload.data.copy_from_slice(slice);
        payload
    }
    pub fn new_from_maccommands<'a>(
        buf: &'a mut [u8],
        mac_commands: &[UplinkMacCommmand],
    ) -> &'a mut Self {
        let res = encode_maccommands(mac_commands, &mut buf[1..]).unwrap();
        let payload_len = res.len();
        Self::mut_from_bytes(&mut buf[..1 + payload_len]).unwrap()
    }
    // pub fn len(&self) -> usize {
    //     1 + self.data.len()
    // }
}

pub trait Mhdr: IntoBytes + Immutable + TryFromBytes {
    fn new(confirmed: bool) -> Self;
    fn dir() -> u8;
}

#[derive(IntoBytes, TryFromBytes, KnownLayout, Unaligned, Immutable)]
#[repr(C, packed)]
pub struct MacPayload<MHDR>
where
    MHDR: Mhdr,
{
    mhdr: MHDR,
    dev_addr: DevAddr,
    f_ctrl: FCtrl,
    f_cnt: U16,
    data: [u8],
}

impl<MHDR> MacPayload<MHDR>
where
    MHDR: Mhdr,
{
    pub fn new(buf: &mut [u8], confirmed: bool, fhdr: FHDR) -> &mut Self {
        let mhdr = MHDR::new(confirmed);
        buf[0] = mhdr.as_bytes()[0];
        let fhdr_len = 7 + fhdr.f_ctrl.f_opts_len();
        buf[1..1 + fhdr_len].copy_from_slice(&fhdr.as_bytes()[..fhdr_len]);
        let uplink = Self::try_mut_from_bytes(buf).unwrap();
        uplink
    }
    pub fn frm_payload(&self) -> &FRMPayload {
        FRMPayload::ref_from_bytes(&self.data[self.f_ctrl.f_opts_len()..]).unwrap()
    }
    pub fn frm_payload_mut(&mut self) -> &mut FRMPayload {
        FRMPayload::mut_from_bytes(&mut self.data[self.f_ctrl.f_opts_len()..]).unwrap()
    }
    pub fn mic(&self) -> MIC {
        MIC::read_from_bytes(&self.data[self.data.len() - 4..]).unwrap()
    }
    pub fn calculate_mic<C: Crypto>(&self, crypto: &mut C, f_cnt: u32, total_len: usize) -> MIC {
        //MIC
        let mut header = [0u8; 16];
        header[0] = 0x49;
        header[5] = MHDR::dir();
        header[6..10].copy_from_slice(self.dev_addr.as_bytes());
        header[10..14].copy_from_slice(&f_cnt.to_le_bytes());
        header[15] = total_len as u8 - 4;
        let mut mac = crypto.get_mac();
        mac.calculate_mic(&[&header, &self.as_bytes()[..total_len - 4]])
    }
    fn encrypt<C: Crypto>(&mut self, crypto: &mut C, f_cnt: u32, payload_len: usize) {
        let mut block = [0u8; 16];
        block[0] = 0x01;
        block[5] = MHDR::dir(); //Dir
        block[6..10].copy_from_slice(self.dev_addr.as_bytes());
        block[10..14].copy_from_slice(&f_cnt.to_le_bytes());
        let payload = self.frm_payload_mut();
        let key = match payload.f_port {
            0 => Key::Network,
            _ => Key::Application,
        };
        let mut encrypter = crypto.get_encrypter(key);
        let mut ctr = 1;
        for i in 0..payload_len {
            let j = i & 0x0f;
            if j == 0 {
                block[15] = ctr;
                ctr += 1;
                encrypter.encrypt_block(&mut block);
            }
            payload.data[i] ^= block[j]
        }
    }
}
