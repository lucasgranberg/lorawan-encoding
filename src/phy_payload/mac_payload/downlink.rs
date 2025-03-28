use zerocopy::{Immutable, IntoBytes, KnownLayout, TryFromBytes, Unaligned};

use crate::mac_command::downlink::DownlinkMacCommandDecoder;

use super::{MacPayload, Mhdr};

#[derive(Debug, TryFromBytes, PartialEq, Eq, KnownLayout, IntoBytes, Unaligned, Immutable)]
#[repr(u8)]
pub enum DownlinkHeader {
    Unconfirmed = 0b01100000,
    Confirmed = 0b10100000,
}
impl Mhdr for DownlinkHeader {
    fn new(confirmed: bool) -> Self {
        match confirmed {
            true => Self::Confirmed,
            false => Self::Unconfirmed,
        }
    }

    fn dir() -> u8 {
        1
    }
}

pub type Downlink = MacPayload<DownlinkHeader>;

impl Downlink {
    pub fn confirmed(&self) -> bool {
        match self.mhdr {
            DownlinkHeader::Unconfirmed => false,
            DownlinkHeader::Confirmed => true,
        }
    }
    pub fn mac_commands(&self) -> Option<DownlinkMacCommandDecoder<'_>> {
        let f_opts_len = self.f_ctrl.f_opts_len();
        if self.f_ctrl.f_opts_len() > 0 {
            Some(DownlinkMacCommandDecoder::new(&self.data[..f_opts_len]))
        } else {
            let payload = self.frm_payload();
            if payload.f_port == 0 {
                Some(DownlinkMacCommandDecoder::new(&payload.data))
            } else {
                None
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use zerocopy::FromBytes as _;

    use crate::{
        crypto::soft::SoftCrypto,
        types::{AppSKey, NwkSKey},
    };

    use super::*;
    fn get_crypto() -> SoftCrypto {
        let nwk_s_key = NwkSKey::read_from_bytes(&[2; 16]).unwrap();
        let app_s_key = AppSKey::read_from_bytes(&[1; 16]).unwrap();
        SoftCrypto::new(nwk_s_key, app_s_key)
    }

    #[test]
    fn decode_downlink() {
        let mut packet = [
            0xa0, 0x04, 0x03, 0x02, 0x01, 0x80, 0xff, 0x2a, 0x2a, 0x0a, 0xf1, 0xa3, 0x6a, 0x05,
            0xd0, 0x12, 0x5f, 0x88, 0x5d, 0x88, 0x1d, 0x49, 0xe1,
        ];
        let mut crypto = get_crypto();
        let downlink = Downlink::new_from_encrypted(&mut packet, 0x12AFF, &mut crypto).unwrap();
        assert_eq!(downlink.mhdr, DownlinkHeader::Confirmed);
        assert_eq!(downlink.dev_addr.as_bytes(), &[4, 3, 2, 1]);
        assert_eq!(downlink.f_cnt.get(), 0x2AFF);
        let f_ctrl = downlink.f_ctrl;
        assert_eq!(f_ctrl.f_opts_len(), 0);
        assert_eq!(f_ctrl.adr(), true);
        assert_eq!(f_ctrl.ack(), false);
        assert_eq!(f_ctrl.f_pending(), false);
        let payload = downlink.frm_payload();
        assert_eq!(payload.data.len(), 14);
        assert_eq!(&payload.data[..payload.data.len() - 4], b"hello lora")
    }
}
