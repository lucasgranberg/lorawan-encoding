#![allow(non_camel_case_types)]
use bitfield_struct::bitfield;
use zerocopy::{Immutable, KnownLayout, TryFromBytes, Unaligned};

use super::{
    DEVICE_TIME_CID, DEV_STATUS_CID, DI_CHANNEL_CID, DUTY_CYCLE_CID, LINK_ADR_CID, LINK_CHECK_CID,
    NEW_CHANNEL_CID, RX_PARAM_SETUP_CID, RX_TIMING_SETUP_CID, TX_PARAM_SETUP_CID,
};

#[derive(Clone, Debug, TryFromBytes, Immutable, KnownLayout, Unaligned)]
#[repr(u8)]
pub enum DownlinkMacCommand {
    LinkCheckAns(LinkCheckAns) = LINK_CHECK_CID,
    LinkADRReq(LinkADRReq) = LINK_ADR_CID,
    DutyCycleReq(DutyCycleReq) = DUTY_CYCLE_CID,
    RXParamSetupReq(RXParamSetupReq) = RX_PARAM_SETUP_CID,
    DevStatusReq(DevStatusReq) = DEV_STATUS_CID,
    NewChannelReq(NewChannelReq) = NEW_CHANNEL_CID,
    RXTimingSetupReq(RXTimingSetupReq) = RX_TIMING_SETUP_CID,
    TxParamSetupReq(TxParamSetupReq) = TX_PARAM_SETUP_CID,
    DIChannelReq(DIChannelReq) = DI_CHANNEL_CID,
    DeviceTimeAns(DeviceTimeAns) = DEVICE_TIME_CID,
}
impl DownlinkMacCommand {
    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        match self {
            DownlinkMacCommand::LinkCheckAns(cmd) => size_of_val(cmd),
            DownlinkMacCommand::LinkADRReq(cmd) => size_of_val(cmd),
            DownlinkMacCommand::DutyCycleReq(cmd) => size_of_val(cmd),
            DownlinkMacCommand::RXParamSetupReq(cmd) => size_of_val(cmd),
            DownlinkMacCommand::DevStatusReq(cmd) => size_of_val(cmd),
            DownlinkMacCommand::NewChannelReq(cmd) => size_of_val(cmd),
            DownlinkMacCommand::RXTimingSetupReq(cmd) => size_of_val(cmd),
            DownlinkMacCommand::TxParamSetupReq(cmd) => size_of_val(cmd),
            DownlinkMacCommand::DIChannelReq(cmd) => size_of_val(cmd),
            DownlinkMacCommand::DeviceTimeAns(cmd) => size_of_val(cmd),
        }
    }
}

#[derive(Clone, Debug, TryFromBytes, Immutable, KnownLayout, Unaligned)]
#[repr(C)]
pub struct LinkCheckAns {
    pub gw_cnt: u8,
    pub margin: u8,
}

#[derive(Clone, Debug, TryFromBytes, Immutable, KnownLayout, Unaligned)]
#[repr(C)]
pub struct LinkADRReq {
    pub data_rate_tx_power: DataRateTXPower,
    pub ch_mask: [u8; 2],
    pub redundancy: Redundancy,
}

#[bitfield(u8)]
#[derive(PartialEq, TryFromBytes, Immutable, KnownLayout, Unaligned)]
pub struct DataRateTXPower {
    #[bits(4)]
    tx_power: u8,
    #[bits(4)]
    data_rate: u8,
}
#[bitfield(u8)]
#[derive(PartialEq, TryFromBytes, Immutable, KnownLayout, Unaligned)]
pub struct Redundancy {
    #[bits(4)]
    nb_trans: u8,
    #[bits(3)]
    ch_mask_cntl: u8,
    _rfu: bool,
}
#[derive(Clone, Debug, TryFromBytes, Immutable, KnownLayout, Unaligned)]
#[repr(C)]
pub struct DutyCycleReq {
    duty_cycle_pl: DutyCyclePl,
}
#[bitfield(u8)]
#[derive(PartialEq, TryFromBytes, Immutable, KnownLayout, Unaligned)]
pub struct DutyCyclePl {
    #[bits(4)]
    max_duty_cycle: u8,
    #[bits(4)]
    _rfu: u8,
}

#[derive(Clone, Debug, PartialEq, TryFromBytes, Immutable, KnownLayout, Unaligned)]
#[repr(C)]
pub struct RXParamSetupReq {
    frequency: [u8; 3],
    dl_settings: DlSettings,
}

#[bitfield(u8)]
#[derive(PartialEq, TryFromBytes, Immutable, KnownLayout, Unaligned)]
pub struct DlSettings {
    #[bits(4)]
    rx2_data_rate: u8,
    #[bits(3)]
    rx1_dr_offset: u8,
    _rfu: bool,
}

#[derive(Clone, Debug, PartialEq, TryFromBytes, Immutable, KnownLayout, Unaligned)]
#[repr(C)]
pub struct DevStatusReq {
    radio_status: RadioStatus,
    battery: u8,
}
#[bitfield(u8)]
#[derive(PartialEq, TryFromBytes, Immutable, KnownLayout, Unaligned)]
pub struct RadioStatus {
    #[bits(6)]
    snr: u8,
    #[bits(2)]
    _rfu: u8,
}

#[derive(Clone, Debug, PartialEq, TryFromBytes, Immutable, KnownLayout, Unaligned)]
#[repr(C)]
pub struct NewChannelReq {
    dr_range: DRRange,
    frequency: [u8; 3],
    ch_index: u8,
}
#[bitfield(u8)]
#[derive(PartialEq, TryFromBytes, Immutable, KnownLayout, Unaligned)]
pub struct DRRange {
    #[bits(4)]
    min_dr: u8,
    #[bits(4)]
    max_dr: u8,
}

#[derive(Clone, Debug, PartialEq, TryFromBytes, Immutable, KnownLayout, Unaligned)]
#[repr(C)]
pub struct RXTimingSetupReq {
    rx_timings_settings: RxTimingSettings,
}
#[bitfield(u8)]
#[derive(PartialEq, TryFromBytes, Immutable, KnownLayout, Unaligned)]
pub struct RxTimingSettings {
    #[bits(3)]
    del: u8,
    #[bits(5)]
    _rfu: u8,
}

#[derive(Clone, Debug, PartialEq, TryFromBytes, Immutable, KnownLayout, Unaligned)]
#[repr(C)]
pub struct TxParamSetupReq {
    eirp_dwell_time: EirpDwellTime,
}
#[bitfield(u8)]
#[derive(PartialEq, TryFromBytes, Immutable, KnownLayout, Unaligned)]
pub struct EirpDwellTime {
    #[bits(4)]
    max_eirp: u8,
    uplink_dwell_time: bool,
    downlink_dwell_time: bool,
    #[bits(2)]
    _rfu: u8,
}

#[derive(Clone, Debug, PartialEq, TryFromBytes, Immutable, KnownLayout, Unaligned)]
#[repr(C)]
pub struct DIChannelReq {
    frequency: [u8; 3],
    ch_index: u8,
}

#[derive(Clone, Debug, PartialEq, TryFromBytes, Immutable, KnownLayout, Unaligned)]
#[repr(C)]
pub struct DeviceTimeAns {
    fractions: u8,
    seconds: [u8; 4],
}
pub struct DownlinkMacCommandDecoder<'a> {
    buf: &'a [u8],
}

impl<'a> DownlinkMacCommandDecoder<'a> {
    pub fn new(buf: &'a [u8]) -> Self {
        Self { buf }
    }
}
impl Iterator for DownlinkMacCommandDecoder<'_> {
    type Item = DownlinkMacCommand;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.buf.is_empty() {
            let remaining = size_of::<DownlinkMacCommand>().min(self.buf.len());
            let mut tmp = [0u8; size_of::<DownlinkMacCommand>()];
            tmp[..remaining].copy_from_slice(&self.buf[..remaining]);
            match DownlinkMacCommand::try_read_from_bytes(&tmp) {
                Ok(r) => {
                    self.buf = &self.buf[r.len() + 1..];
                    Some(r)
                }
                Err(_) => {
                    self.buf = &[];
                    None
                }
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn decode_downlink_cmds() {
        let buf = [0x02, 0x03, 0x04, 0x03, 0x21, 0x02, 0x03, 0x45];
        let decoder = DownlinkMacCommandDecoder::new(&buf);
        let cmds: Vec<_> = decoder.collect();
        assert_eq!(2, cmds.len());
        assert!(matches!(
            cmds.get(0),
            Some(DownlinkMacCommand::LinkCheckAns(LinkCheckAns {
                gw_cnt: 3,
                margin: 4
            }))
        ));
        if let Some(DownlinkMacCommand::LinkADRReq(LinkADRReq {
            data_rate_tx_power,
            ch_mask,
            redundancy,
        })) = cmds.get(1)
        {
            assert_eq!(
                data_rate_tx_power,
                &DataRateTXPower::new().with_tx_power(1).with_data_rate(2)
            );
            assert_eq!(ch_mask, &[0x02, 0x03]);
            assert_eq!(
                redundancy,
                &Redundancy::new().with_ch_mask_cntl(4).with_nb_trans(5)
            );
        } else {
            assert!(false, "Wrong command type: {:?}", cmds.get(1))
        }
        assert!(matches!(cmds.get(2), None));
    }
}
