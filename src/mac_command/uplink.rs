#![allow(non_camel_case_types)]
use bitfield_struct::bitfield;
use zerocopy::{Immutable, IntoBytes, KnownLayout};

use crate::Error;

use super::{
    DEVICE_TIME_CID, DEV_STATUS_CID, DI_CHANNEL_CID, DUTY_CYCLE_CID, LINK_ADR_CID, LINK_CHECK_CID,
    NEW_CHANNEL_CID, RX_PARAM_SETUP_CID, RX_TIMING_SETUP_CID, TX_PARAM_SETUP_CID,
};

#[derive(IntoBytes, Immutable, KnownLayout)]
#[repr(u8)]
pub enum UplinkMacCommmand {
    LinkCheckReq(LinkCheckReq) = LINK_CHECK_CID,
    LinkADRAns(LinkADRAns) = LINK_ADR_CID,
    DutyCycleAns(DutyCycleAns) = DUTY_CYCLE_CID,
    RXParamSetupAns(RXParamSetupAns) = RX_PARAM_SETUP_CID,
    DevStatusAns(DevStatusAns) = DEV_STATUS_CID,
    NewChannelAns(NewChannelAns) = NEW_CHANNEL_CID,
    RXTimingSetupAns(RXTimingSetupAns) = RX_TIMING_SETUP_CID,
    TxParamSetupAns(TxParamSetupAns) = TX_PARAM_SETUP_CID,
    DIChannelAns(DIChannelAns) = DI_CHANNEL_CID,
    DeviceTimeReq(DeviceTimeReq) = DEVICE_TIME_CID,
}
#[allow(clippy::len_without_is_empty)]
impl UplinkMacCommmand {
    pub fn len(&self) -> usize {
        match self {
            UplinkMacCommmand::LinkCheckReq(cmd) => size_of_val(cmd) - size_of_val(&cmd._padding),
            UplinkMacCommmand::LinkADRAns(cmd) => size_of_val(cmd) - size_of_val(&cmd._padding),
            UplinkMacCommmand::DutyCycleAns(cmd) => size_of_val(cmd) - size_of_val(&cmd._padding),
            UplinkMacCommmand::RXParamSetupAns(cmd) => {
                size_of_val(cmd) - size_of_val(&cmd._padding)
            }
            UplinkMacCommmand::DevStatusAns(cmd) => size_of_val(cmd) - size_of_val(&cmd._padding),
            UplinkMacCommmand::NewChannelAns(cmd) => size_of_val(cmd) - size_of_val(&cmd._padding),
            UplinkMacCommmand::RXTimingSetupAns(cmd) => {
                size_of_val(cmd) - size_of_val(&cmd._padding)
            }
            UplinkMacCommmand::TxParamSetupAns(cmd) => {
                size_of_val(cmd) - size_of_val(&cmd._padding)
            }
            UplinkMacCommmand::DIChannelAns(cmd) => size_of_val(cmd) - size_of_val(&cmd._padding),
            UplinkMacCommmand::DeviceTimeReq(cmd) => size_of_val(cmd) - size_of_val(&cmd._padding),
        }
    }
}

#[derive(Default, IntoBytes, Immutable, KnownLayout)]
#[repr(C)]
pub struct LinkCheckReq {
    _padding: [u8; 2],
}
impl LinkCheckReq {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}

#[derive(Default, IntoBytes, Immutable, KnownLayout)]
#[repr(C)]
pub struct LinkADRAns {
    pub status: LinkAdrAnsStatus,
    _padding: [u8; 1],
}
impl LinkADRAns {
    pub fn new(status: LinkAdrAnsStatus) -> Self {
        Self {
            status,
            ..Default::default()
        }
    }
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
pub struct DutyCycleAns {
    _padding: [u8; 2],
}
#[derive(IntoBytes, Immutable, KnownLayout)]
#[repr(C)]
pub struct RXParamSetupAns {
    status: RXParamSetupAnsStatus,
    _padding: [u8; 1],
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
    radio_status: u8,
    battery: u8,
    _padding: [u8; 0],
}
#[derive(IntoBytes, Immutable, KnownLayout)]
#[repr(C)]
pub struct NewChannelAns {
    status: NewChannelAnsStatus,
    _padding: [u8; 1],
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
    _padding: [u8; 1],
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
pub struct TxParamSetupAns {
    _padding: [u8; 2],
}
#[derive(IntoBytes, Immutable, KnownLayout)]
#[repr(C)]
pub struct DIChannelAns {
    status: DIChannelAnsStatus,
    _padding: [u8; 1],
}
#[bitfield(u8)]
#[derive(IntoBytes, Immutable, KnownLayout)]
pub struct DIChannelAnsStatus {
    pub channel_frequency_ok: bool,
    pub uplink_frequency_exists: bool,
    #[bits(6)]
    _rfu: u8,
}

#[derive(IntoBytes, Immutable, KnownLayout)]
#[repr(C)]
pub struct DeviceTimeReq {
    _padding: [u8; 2],
}

pub fn encode_maccommands<'a>(
    cmds: &[UplinkMacCommmand],
    buf: &'a mut [u8],
) -> Result<&'a [u8], Error> {
    let mut pos = 0usize;
    for cmd in cmds {
        let len = cmd.len() + 1;
        buf[pos..pos + len].copy_from_slice(&cmd.as_bytes()[..len]);
        pos += len;
    }
    Ok(&buf[..pos])
}
pub fn get_mac_commands_len(cmds: &[UplinkMacCommmand]) -> usize {
    let mut pos = 0usize;
    for cmd in cmds {
        let len = cmd.len() + 1;
        pos += len;
    }
    pos
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn encode_uplink_cmds() {
        let cmds = vec![
            UplinkMacCommmand::LinkCheckReq(LinkCheckReq::new()),
            UplinkMacCommmand::LinkADRAns(LinkADRAns::new(LinkAdrAnsStatus::new())),
        ];
        let mut buf = [0u8; 255];
        let cmd_buf = encode_maccommands(cmds.as_slice(), &mut buf).unwrap();
        assert_eq!(cmd_buf, &[0x02, 0x03, 0x00])
    }
}
