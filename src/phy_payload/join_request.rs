use zerocopy::{Immutable, IntoBytes, KnownLayout};

use crate::types::{DevEui, DevNonce, JoinEui};

#[derive(IntoBytes, KnownLayout, Immutable)]
#[repr(u8)]
pub enum MHDR {
    JoinRequest = 0b00000000,
}
#[derive(IntoBytes, KnownLayout, Immutable)]
pub struct JoinRequest {
    _mhdr: MHDR,
    pub join_eui: JoinEui,
    pub dev_eui: DevEui,
    pub dev_nonce: DevNonce,
}

impl JoinRequest {
    pub fn new(join_eui: JoinEui, dev_eui: DevEui, dev_nonce: DevNonce) -> Self {
        Self {
            _mhdr: MHDR::JoinRequest,
            join_eui,
            dev_eui,
            dev_nonce,
        }
    }
}
#[cfg(test)]
mod tests {
    use zerocopy::FromBytes;

    use super::*;
    #[test]
    fn encode_join_request() {
        let join_request = JoinRequest::new(
            JoinEui::read_from_bytes(&[0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08]).unwrap(),
            DevEui::read_from_bytes(&[0x10, 0x20, 0x30, 0x40, 0x50, 0x60, 0x70, 0x80]).unwrap(),
            DevNonce::read_from_bytes(&[0x11, 0x22]).unwrap(),
        );
        assert_eq!(
            join_request.as_bytes(),
            &[
                0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x10, 0x20, 0x30, 0x40, 0x50,
                0x60, 0x70, 0x80, 0x11, 0x22
            ]
        );
        assert_eq!(join_request.dev_nonce.as_bytes(), &[0x11, 0x22]);
    }
}
