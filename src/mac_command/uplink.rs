#![allow(non_camel_case_types)]
use bitfield_struct::bitfield;
use zerocopy::{Immutable, IntoBytes, KnownLayout};

use crate::Error;

use super::{
    DEVICE_TIME_CID, DEV_STATUS_CID, DI_CHANNEL_CID, DUTY_CYCLE_CID, LINK_ADR_CID, LINK_CHECK_CID,
    NEW_CHANNEL_CID, RX_PARAM_SETUP_CID, RX_TIMING_SETUP_CID, TX_PARAM_SETUP_CID,
};

#[repr(u8)]
pub enum UplinkMacCommmand {
    LinkCheckReq = LINK_CHECK_CID,
    LinkADRAns(LinkADRAns) = LINK_ADR_CID,
    DutyCycleAns = DUTY_CYCLE_CID,
    RXParamSetupAns(RXParamSetupAns) = RX_PARAM_SETUP_CID,
    DevStatusAns(DevStatusAns) = DEV_STATUS_CID,
    NewChannelAns(NewChannelAns) = NEW_CHANNEL_CID,
    RXTimingSetupAns(RXTimingSetupAns) = RX_TIMING_SETUP_CID,
    TxParamSetupAns = TX_PARAM_SETUP_CID,
    DlChannelAns(DIChannelAns) = DI_CHANNEL_CID,
    DeviceTimeReq = DEVICE_TIME_CID,
}
#[allow(clippy::len_without_is_empty)]
impl UplinkMacCommmand {
    // https://doc.rust-lang.org/reference/items/enumerations.html#pointer-casting
    fn cid(&self) -> u8 {
        unsafe { *((self as *const Self) as *const u8) }
    }
    pub fn as_bytes(&self) -> &[u8] {
        match self {
            UplinkMacCommmand::LinkCheckReq => &[],
            UplinkMacCommmand::LinkADRAns(cmd) => cmd.as_bytes(),
            UplinkMacCommmand::DutyCycleAns => &[],
            UplinkMacCommmand::RXParamSetupAns(cmd) => cmd.as_bytes(),
            UplinkMacCommmand::DevStatusAns(cmd) => cmd.as_bytes(),
            UplinkMacCommmand::NewChannelAns(cmd) => cmd.as_bytes(),
            UplinkMacCommmand::RXTimingSetupAns(cmd) => cmd.as_bytes(),
            UplinkMacCommmand::TxParamSetupAns => &[],
            UplinkMacCommmand::DlChannelAns(cmd) => cmd.as_bytes(),
            UplinkMacCommmand::DeviceTimeReq => &[],
        }
    }
}

#[derive(Default, IntoBytes, Immutable, KnownLayout)]
#[repr(C)]
pub struct LinkADRAns {
    pub status: LinkAdrAnsStatus,
}

#[bitfield(u8)]
#[derive(IntoBytes, Immutable, KnownLayout)]
pub struct LinkAdrAnsStatus {
    pub channel_mask_ack: bool,
    pub data_rate_ack: bool,
    pub power_ack: bool,
    #[bits(5)]
    _rfu: u8,
}

#[derive(IntoBytes, Immutable, KnownLayout)]
#[repr(C)]
pub struct RXParamSetupAns {
    status: RXParamSetupAnsStatus,
}
#[bitfield(u8)]
#[derive(IntoBytes, Immutable, KnownLayout)]
pub struct RXParamSetupAnsStatus {
    pub channel_ack: bool,
    pub rx2_data_rate_ack: bool,
    pub rx1_data_rate_offset_ack: bool,
    #[bits(5)]
    _rfu: u8,
}

#[derive(IntoBytes, Immutable, KnownLayout)]
#[repr(C)]
pub struct DevStatusAns {
    radio_status: DevStatusAnsRadioStatus,
    battery: u8,
}
#[bitfield(u8)]
#[derive(IntoBytes, Immutable, KnownLayout)]
pub struct DevStatusAnsRadioStatus {
    #[bits(6)]
    snr: i8,
    #[bits(2)]
    _rfu: u8,
}
#[derive(IntoBytes, Immutable, KnownLayout)]
#[repr(C)]
pub struct NewChannelAns {
    status: NewChannelAnsStatus,
}
#[bitfield(u8)]
#[derive(IntoBytes, Immutable, KnownLayout)]
pub struct NewChannelAnsStatus {
    pub channel_freq_ok: bool,
    pub data_rate_range_ok: bool,
    #[bits(6)]
    _rfu: u8,
}

#[derive(IntoBytes, Immutable, KnownLayout)]
#[repr(C)]
pub struct RXTimingSetupAns {
    rx_timings_settings: RxTimingsSetting,
}
#[bitfield(u8)]
#[derive(IntoBytes, Immutable, KnownLayout)]
pub struct RxTimingsSetting {
    #[bits(3)]
    pub del: u8,
    #[bits(5)]
    _rfu: u8,
}

#[derive(IntoBytes, Immutable, KnownLayout)]
#[repr(C)]
pub struct DIChannelAns {
    status: DIChannelAnsStatus,
}

#[bitfield(u8)]
#[derive(IntoBytes, Immutable, KnownLayout)]
pub struct DIChannelAnsStatus {
    pub channel_frequency_ok: bool,
    pub uplink_frequency_exists: bool,
    #[bits(6)]
    _rfu: u8,
}

pub fn encode_maccommands<'a>(
    cmds: &[UplinkMacCommmand],
    buf: &'a mut [u8],
) -> Result<&'a [u8], Error> {
    let mut pos = 0usize;
    for cmd in cmds {
        let bytes = cmd.as_bytes();
        let len = bytes.len() + 1;
        if pos + len > buf.len() {
            return Err(Error::Size);
        }
        buf[pos] = cmd.cid();
        if len > 1 {
            buf[pos + 1..pos + len].copy_from_slice(bytes);
        }
        pos += len
    }
    Ok(&buf[..pos])
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn encode_uplink_cmds() {
        let cmds = [
            UplinkMacCommmand::LinkCheckReq,
            UplinkMacCommmand::LinkADRAns(LinkADRAns {
                status: LinkAdrAnsStatus::new(),
            }),
            UplinkMacCommmand::DutyCycleAns,
            UplinkMacCommmand::RXParamSetupAns(RXParamSetupAns {
                status: RXParamSetupAnsStatus::new(),
            }),
            UplinkMacCommmand::DevStatusAns(DevStatusAns {
                radio_status: DevStatusAnsRadioStatus::new(),
                battery: 112,
            }),
            UplinkMacCommmand::NewChannelAns(NewChannelAns {
                status: NewChannelAnsStatus::new(),
            }),
            UplinkMacCommmand::RXTimingSetupAns(RXTimingSetupAns {
                rx_timings_settings: RxTimingsSetting::new().with_del(3),
            }),
            UplinkMacCommmand::TxParamSetupAns,
            UplinkMacCommmand::DlChannelAns(DIChannelAns {
                status: DIChannelAnsStatus::new(),
            }),
            UplinkMacCommmand::DeviceTimeReq,
        ];
        let mut buf = [0u8; 255];
        let cmd_buf = encode_maccommands(&cmds, &mut buf).unwrap();
        assert_eq!(
            cmd_buf,
            &[
                0x02, 0x03, 0x00, 0x04, 0x05, 0x00, 0x06, 0, 112, 0x07, 0x00, 0x08, 0x03, 0x09,
                0x0A, 0x00, 0x0D
            ]
        )
    }
    #[test]
    fn encode_uplink_dev_status_ans() {
        let cmds = [
            UplinkMacCommmand::DevStatusAns(DevStatusAns {
                radio_status: DevStatusAnsRadioStatus::new().with_snr(-15),
                battery: 112,
            }),
            UplinkMacCommmand::DevStatusAns(DevStatusAns {
                radio_status: DevStatusAnsRadioStatus::new().with_snr(0),
                battery: 225,
            }),
            UplinkMacCommmand::DevStatusAns(DevStatusAns {
                radio_status: DevStatusAnsRadioStatus::new().with_snr(15),
                battery: 0,
            }),
        ];
        let mut buf = [0u8; 255];
        let cmd_buf = encode_maccommands(&cmds, &mut buf).unwrap();
        assert_eq!(cmd_buf, &[0x06, 49, 112, 0x06, 0, 225, 0x06, 15, 0])
    }
    #[test]
    fn encode_uplink_rx_param_setup_ans() {
        let cmds = [
            UplinkMacCommmand::RXParamSetupAns(RXParamSetupAns {
                status: RXParamSetupAnsStatus::new()
                    .with_channel_ack(true)
                    .with_rx2_data_rate_ack(false)
                    .with_rx1_data_rate_offset_ack(false),
            }),
            UplinkMacCommmand::RXParamSetupAns(RXParamSetupAns {
                status: RXParamSetupAnsStatus::new()
                    .with_channel_ack(false)
                    .with_rx2_data_rate_ack(true)
                    .with_rx1_data_rate_offset_ack(false),
            }),
            UplinkMacCommmand::RXParamSetupAns(RXParamSetupAns {
                status: RXParamSetupAnsStatus::new()
                    .with_channel_ack(false)
                    .with_rx2_data_rate_ack(false)
                    .with_rx1_data_rate_offset_ack(true),
            }),
        ];
        let mut buf = [0u8; 255];
        let cmd_buf = encode_maccommands(&cmds, &mut buf).unwrap();
        assert_eq!(cmd_buf, &[0x05, 0x01, 0x05, 0x02, 0x05, 0x04])
    }
    #[test]
    fn encode_uplink_new_channel_ans() {
        let cmds = [
            UplinkMacCommmand::NewChannelAns(NewChannelAns {
                status: NewChannelAnsStatus::new()
                    .with_channel_freq_ok(true)
                    .with_data_rate_range_ok(false),
            }),
            UplinkMacCommmand::NewChannelAns(NewChannelAns {
                status: NewChannelAnsStatus::new()
                    .with_channel_freq_ok(false)
                    .with_data_rate_range_ok(true),
            }),
        ];
        let mut buf = [0u8; 255];
        let cmd_buf = encode_maccommands(&cmds, &mut buf).unwrap();
        assert_eq!(cmd_buf, &[0x07, 0x01, 0x07, 0x02])
    }
    #[test]
    fn encode_uplink_dl_channel_ans() {
        let cmds = [
            UplinkMacCommmand::DlChannelAns(DIChannelAns {
                status: DIChannelAnsStatus::new()
                    .with_channel_frequency_ok(true)
                    .with_uplink_frequency_exists(false),
            }),
            UplinkMacCommmand::DlChannelAns(DIChannelAns {
                status: DIChannelAnsStatus::new()
                    .with_channel_frequency_ok(false)
                    .with_uplink_frequency_exists(true),
            }),
        ];
        let mut buf = [0u8; 255];
        let cmd_buf = encode_maccommands(&cmds, &mut buf).unwrap();
        assert_eq!(cmd_buf, &[0x0A, 0x01, 0x0A, 0x02])
    }
}
