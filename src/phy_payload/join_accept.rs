use bitfield_struct::bitfield;
use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout, TryFromBytes, Unaligned};

#[derive(TryFromBytes, IntoBytes, KnownLayout, Immutable, Unaligned)]
#[repr(u8)]
pub enum JoinAcceptHeader {
    JoinAccept = 0b00100000,
}

#[derive(TryFromBytes, KnownLayout, Immutable)]
#[repr(C)]
pub struct JoinAccept {
    mhdr: JoinAcceptHeader,
    join_nonce: [u8; 3],
    net_id: [u8; 3],
    dev_addr: [u8; 4],
    dl_settings: DlSettings,
    rx_delay: u8,
    cf_list: [u8],
}

#[bitfield(u8)]
#[derive(PartialEq, FromBytes, IntoBytes, KnownLayout, Immutable, Unaligned)]
pub struct DlSettings {
    #[bits(4)]
    rx2_dr: u8,
    #[bits(3)]
    rx1_dr_offset: u8,
    #[bits(1)]
    _rfu: bool,
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn decode_join_accept() {
        let (join_accept, _) = JoinAccept::try_ref_from_prefix(&[
            0x20, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x55, 0x0c,
        ])
        .unwrap();
        assert_eq!(&join_accept.join_nonce, &[0x01, 0x02, 0x03]);
        assert_eq!(&join_accept.net_id, &[0x04, 0x05, 0x06]);
        assert_eq!(&join_accept.dev_addr, &[0x07, 0x08, 0x09, 0x0a]);
        assert_eq!(
            &join_accept.dl_settings,
            &DlSettings::new().with_rx1_dr_offset(5).with_rx2_dr(5)
        );
        assert_eq!(join_accept.rx_delay, 0x0c);
    }
}
