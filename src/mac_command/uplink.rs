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
    // https://doc.rust-lang.org/reference/items/enumerations.html#pointer-casting
    fn cid(&self) -> u8 {
        unsafe { *(self as *const Self as *const u8) }
    }
    pub fn as_bytes(&self) -> &[u8] {
        match self {
            UplinkMacCommmand::LinkCheckReq(cmd) => cmd.as_bytes(),
            UplinkMacCommmand::LinkADRAns(cmd) => cmd.as_bytes(),
            UplinkMacCommmand::DutyCycleAns(cmd) => cmd.as_bytes(),
            UplinkMacCommmand::RXParamSetupAns(cmd) => cmd.as_bytes(),
            UplinkMacCommmand::DevStatusAns(cmd) => cmd.as_bytes(),
            UplinkMacCommmand::NewChannelAns(cmd) => cmd.as_bytes(),
            UplinkMacCommmand::RXTimingSetupAns(cmd) => cmd.as_bytes(),
            UplinkMacCommmand::TxParamSetupAns(cmd) => cmd.as_bytes(),
            UplinkMacCommmand::DIChannelAns(cmd) => cmd.as_bytes(),
            UplinkMacCommmand::DeviceTimeReq(cmd) => cmd.as_bytes(),
        }
    }
}

#[derive(Default, IntoBytes, Immutable, KnownLayout)]
#[repr(C)]
pub struct LinkCheckReq {}
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
pub struct DutyCycleAns {}
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
    radio_status: u8,
    battery: u8,
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
pub struct TxParamSetupAns {}
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

#[derive(IntoBytes, Immutable, KnownLayout)]
#[repr(C)]
pub struct DeviceTimeReq {}

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
            UplinkMacCommmand::LinkCheckReq(LinkCheckReq::new()),
            UplinkMacCommmand::LinkADRAns(LinkADRAns::new(LinkAdrAnsStatus::new())),
        ];
        let mut buf = [0u8; 255];
        let cmd_buf = encode_maccommands(&cmds, &mut buf).unwrap();
        assert_eq!(cmd_buf, &[0x02, 0x03, 0x00])
    }
}
