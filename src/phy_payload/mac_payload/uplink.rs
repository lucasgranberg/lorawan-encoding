use zerocopy::{Immutable, IntoBytes, KnownLayout, TryFromBytes, Unaligned};

use crate::crypto::Crypto;

use super::{FRMPayload, MacPayload, Mhdr};

#[derive(IntoBytes, TryFromBytes, KnownLayout, Unaligned, Immutable)]
#[repr(u8)]
pub enum UplinkHeader {
    Unconfirmed = 0b01000000,
    Confirmed = 0b10000000,
}
impl Mhdr for UplinkHeader {
    fn new(confirmed: bool) -> Self {
        match confirmed {
            true => Self::Confirmed,
            false => Self::Unconfirmed,
        }
    }

    fn dir() -> u8 {
        0
    }
}

pub type Uplink = MacPayload<UplinkHeader>;

impl Uplink {
    pub fn build<C, F>(&mut self, f_cnt: u32, crypto: &mut C, c: F) -> &[u8]
    where
        C: Crypto,
        F: FnOnce(&mut [u8]) -> Option<&mut FRMPayload>,
    {
        let f_opts_len = self.f_ctrl.f_opts_len();
        let data_len = self.data.len();
        //mhdr + f_opts - MIC
        let payload_len = if let Some(payload) = c(&mut self.data[f_opts_len..data_len - 4]) {
            1 + payload.data.len()
        } else {
            0
        };
        self.encrypt(crypto, f_cnt, payload_len);
        let total_len = 8 + f_opts_len + payload_len + 4;

        let mic = self.calculate_mic(crypto, f_cnt, total_len);
        self.data[f_opts_len + payload_len..f_opts_len + payload_len + 4]
            .copy_from_slice(mic.as_bytes());

        //mhdr + fhdr + f_opts + payload + mic
        &self.as_bytes()[..total_len]
    }
}
#[cfg(test)]
mod tests {
    use zerocopy::FromBytes;

    use crate::{
        crypto::soft::SoftCrypto,
        mac_command::uplink::{
            encode_maccommands, LinkADRAns, LinkAdrAnsStatus, LinkCheckReq, UplinkMacCommmand,
        },
        phy_payload::mac_payload::FHDR,
        types::{AppSKey, DevAddr, NwkSKey},
    };
    fn get_crypto() -> SoftCrypto {
        let nwk_s_key = NwkSKey::read_from_bytes(&[1; 16]).unwrap();
        let app_s_key = AppSKey::read_from_bytes(&[0; 16]).unwrap();
        SoftCrypto::new(nwk_s_key, app_s_key)
    }
    use super::*;
    #[test]
    fn encode_uplink() {
        let mut buf = [0u8; 256];
        let payload: [u8; 2] = [0x08, 0x09];
        let fhdr = FHDR::new(
            DevAddr::read_from_bytes(&[0, 1, 2, 3]).unwrap(),
            true,
            false,
            false,
            0x0506,
            &[],
        )
        .unwrap();
        let uplink = Uplink::new(&mut buf, true, fhdr);
        let bytes = uplink.build(0, &mut get_crypto(), |buf| {
            Some(FRMPayload::new_from_slice(buf, 7, &payload))
        });
        assert_eq!(
            bytes,
            &[128, 0, 1, 2, 3, 128, 6, 5, 7, 255, 123, 34, 224, 206, 195]
        )
    }
    #[test]
    fn encode_empty_uplink() {
        let mut buf = [0u8; 256];
        let fhdr = FHDR::new(
            DevAddr::read_from_bytes(&[0, 1, 2, 3]).unwrap(),
            true,
            false,
            false,
            0x0506,
            &[],
        )
        .unwrap();
        let uplink = Uplink::new(&mut buf, true, fhdr);
        let bytes = uplink.build(0, &mut get_crypto(), |_buf| None);
        assert_eq!(bytes, &[128, 0, 1, 2, 3, 128, 6, 5, 75, 31, 216, 35])
    }
    #[test]
    fn encode_uplink_with_fopts() {
        let mut buf = [0u8; 256];
        let cmds = [
            UplinkMacCommmand::LinkCheckReq(LinkCheckReq::new()),
            UplinkMacCommmand::LinkADRAns(LinkADRAns::new(
                LinkAdrAnsStatus::new()
                    .with_power_ack(true)
                    .with_channel_mask_ack(true),
            )),
        ];
        let mut f_opts_buf = [0u8; 15];
        let fhdr = FHDR::new(
            DevAddr::read_from_bytes(&[4, 3, 2, 1]).unwrap(),
            false,
            false,
            false,
            0,
            encode_maccommands(&cmds, &mut f_opts_buf).unwrap(),
        )
        .unwrap();
        let uplink = Uplink::new(&mut buf, false, fhdr);
        assert_eq!(
            uplink.build(0, &mut get_crypto(), |_| { None }),
            &[
                0x40, 0x04, 0x03, 0x02, 0x01, 0x03, 0x00, 0x00, 0x02, 0x03, 0x05, 0xd7, 0xfa, 0x0c,
                0x6c
            ]
        );
    }
    #[test]
    fn encrypt_uplink() {
        let mut buf = [0u8; 256];
        let cmds = [
            UplinkMacCommmand::LinkCheckReq(LinkCheckReq::new()),
            UplinkMacCommmand::LinkADRAns(LinkADRAns::new(
                LinkAdrAnsStatus::new()
                    .with_power_ack(true)
                    .with_channel_mask_ack(true),
            )),
        ];
        let fhdr = FHDR::new(
            DevAddr::read_from_bytes(&[4, 3, 2, 1]).unwrap(),
            false,
            false,
            false,
            0,
            &[],
        )
        .unwrap();
        let uplink = Uplink::new(&mut buf, false, fhdr);
        let bytes = uplink.build(0, &mut get_crypto(), |buf| {
            Some(FRMPayload::new_from_maccommands(buf, &cmds))
        });
        assert_eq!(
            bytes,
            &[
                0x40, 0x04, 0x03, 0x02, 0x01, 0x00, 0x00, 0x00, 0x00, 0x69, 0x36, 0x9e, 0xee, 0x6a,
                0xa5, 0x08
            ]
        )
    }

    // #[test]
    // fn encode_uplink_with_mac_commands_in_payload() {
    //     let mut buf = [0u8; 255];
    //     let fhdr = FHDR::new(
    //         DevAddr::read_from_bytes(&[0, 1, 2, 3]).unwrap(),
    //         true,
    //         false,
    //         false,
    //         0x0506,
    //         &[],
    //     )
    //     .unwrap();
    //     let uplink = Uplink::new(&mut buf, true, fhdr);
    //     let bytes = uplink.build(|_buf| FRMPayload);
    //     assert_eq!(bytes, &[128, 0, 1, 2, 3, 128, 5, 6, 0, 0, 0, 0])
    // }
}
