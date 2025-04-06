use zerocopy::{little_endian::U16, Immutable, KnownLayout, TryFromBytes, Unaligned};

use crate::Error;

use super::cid::*;

#[allow(non_camel_case_types)]
#[derive(PartialEq, Debug)]
#[repr(u8)]
pub enum CertificationDownlinkMacCommand<'a> {
    /// Used by the TCL to request the package version implemented by the end-device
    PackageVersionReq = PACKET_VERSION_CID,
    /// DUT SHALL reset the MCU
    DutResetReq = DUT_RESET_CID,
    /// DUT SHALL start issuing Join-Request messages
    DutJoinReq = DUT_JOIN_CID,
    /// DUT SHALL change its Class of operation to A, B or C
    SwitchClassReq(&'a SwitchClassReq) = SWITCH_CLASS_CID,
    /// DUT SHALL activate/deactivate ADR
    AdrBitChangeReq(bool) = ADR_BIT_CHANGE_CID,
    /// DUT SHALL activate/deactivate the regional band duty-cycle enforcement
    RegionalDutyCycleCtrlReq(bool) = REGIONAL_DUTY_CYCLE_CID,
    /// DUT SHALL change its uplink periodicity to the provided value
    TxPeriodicityChangeReq(u8) = TX_PERIODICITY_CHANGE_CID,
    /// All subsequent DUT uplinks SHALL be of specified type
    TxFramesCtrlReq(&'a TxFramesCtrlReq) = TX_FRAMES_CTRL_CID,
    /// TCL requests the DUT to echo the provided payload where each byte is incremented by 1
    EchoPayloadReq(&'a [u8]) = ECHO_PAYLOAD_CID,
    /// TCL requests the DUT to provide the current applicative RxAppCnt value
    RxAppCntReq(&'a U16) = RX_APP_CNT_CID,
    /// DUT SHALL reset the applicative RxAppCnt value to 0
    RxAppCntResetReq = RX_APP_CNT_RESET_CID,
    /// DUT SHALL send a LinkCheckReq MAC command to the TCL
    LinkCheckReq = LINK_CHECK_CID,
    /// DUT SHALL send a DeviceTimeReq MAC command to the TCL
    DeviceTimeReq = DEVICE_TIME_CID,
    /// DUT SHALL send a PingSlotInfoReq MAC command to the TCL Only required for Class B DUT
    PingSlotInfoReq(u8) = PING_SLOT_INFO_CID,
    /// DUT SHALL set the radio in continuous wave transmission mode
    TxCwReq = TX_CW_CID,
    /// DUT SHALL disable the processing of data received on FPort 224
    DutFPort224DisableReq = DUT_FPORT_224_DISABLE_CID,
    /// TCL requests the DUT to send its firmware version, LoRaWAN version and Regional parameters version
    DutVersionsReq = DUT_VERSION_CID,
}

impl<'a> CertificationDownlinkMacCommand<'a> {
    pub fn decode(buf: &'a [u8]) -> Result<Self, Error> {
        if buf.is_empty() {
            return Err(Error::Size);
        }
        match buf[0] {
            PACKET_VERSION_CID => Ok(Self::PackageVersionReq),
            DUT_RESET_CID => Ok(Self::DutResetReq),
            DUT_JOIN_CID => Ok(Self::DutJoinReq),
            SWITCH_CLASS_CID => Ok(Self::SwitchClassReq(
                &TryFromBytes::try_ref_from_prefix(&buf[1..])
                    .map_err(|_| Error::Payload)?
                    .0,
            )),
            ADR_BIT_CHANGE_CID => Ok(Self::AdrBitChangeReq(buf[1] == 1)),
            REGIONAL_DUTY_CYCLE_CID => Ok(Self::RegionalDutyCycleCtrlReq(buf[1] == 1)),
            TX_PERIODICITY_CHANGE_CID => Ok(Self::TxPeriodicityChangeReq(buf[1])),
            TX_FRAMES_CTRL_CID => Ok(Self::TxFramesCtrlReq(
                TryFromBytes::try_ref_from_prefix(&buf[1..])
                    .map_err(|_| Error::Payload)?
                    .0,
            )),
            ECHO_PAYLOAD_CID => Ok(Self::EchoPayloadReq(&buf[1..])),
            RX_APP_CNT_CID => Ok(Self::RxAppCntReq(
                TryFromBytes::try_ref_from_prefix(&buf[1..])
                    .map_err(|_| Error::Payload)?
                    .0,
            )),
            RX_APP_CNT_RESET_CID => Ok(Self::RxAppCntResetReq),
            LINK_CHECK_CID => Ok(Self::LinkCheckReq),
            DEVICE_TIME_CID => Ok(Self::DeviceTimeReq),
            PING_SLOT_INFO_CID => Ok(Self::PingSlotInfoReq(buf[1])),
            TX_CW_CID => Ok(Self::TxCwReq),
            DUT_FPORT_224_DISABLE_CID => Ok(Self::DutFPort224DisableReq),
            DUT_VERSION_CID => Ok(Self::DutVersionsReq),
            _ => Err(Error::Payload),
        }
    }
    // pub fn len(&self) -> usize {
    //     match self {
    //         CertificationDownlinkMacCommand::PackageVersionReq => 0,
    //         CertificationDownlinkMacCommand::DutResetReq => 0,
    //         CertificationDownlinkMacCommand::DutJoinReq => 0,
    //         CertificationDownlinkMacCommand::SwitchClassReq(_) => 0,
    //         CertificationDownlinkMacCommand::AdrBitChangeReq(_) => 1,
    //         CertificationDownlinkMacCommand::RegionalDutyCycleCtrlReq(_) => 1,
    //         CertificationDownlinkMacCommand::TxPeriodicityChangeReq(_) => 1,
    //         CertificationDownlinkMacCommand::TxFramesCtrlReq(_) => 1,
    //         CertificationDownlinkMacCommand::EchoPayloadReq(_) => 0,
    //         CertificationDownlinkMacCommand::RxAppCntReq(_) => 2,
    //         CertificationDownlinkMacCommand::RxAppCntResetReq => 0,
    //         CertificationDownlinkMacCommand::LinkCheckReq => 0,
    //         CertificationDownlinkMacCommand::DeviceTimeReq => 0,
    //         CertificationDownlinkMacCommand::PingSlotInfoReq(_) => 0,
    //         CertificationDownlinkMacCommand::TxCwReq => 0,
    //         CertificationDownlinkMacCommand::DutFPort224DisableReq => 0,
    //         CertificationDownlinkMacCommand::DutVersionsReq => 0,
    //     }
    // }
}

#[derive(Clone, Debug, PartialEq, TryFromBytes, Immutable, KnownLayout, Unaligned)]
#[repr(u8)]
pub enum SwitchClassReq {
    A = 0,
    B = 1,
    C = 2,
}

#[derive(Clone, Debug, PartialEq, TryFromBytes, Immutable, KnownLayout, Unaligned)]
#[repr(C)]
pub struct TxFramesCtrlReq {
    frame_type: TxFramesCtrlReqFrameType,
}

#[derive(Clone, Debug, PartialEq, TryFromBytes, Immutable, KnownLayout, Unaligned)]
#[repr(u8)]
pub enum TxFramesCtrlReqFrameType {
    NoChange = 0,
    Unconfirmed = 1,
    Confirmed = 2,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn decode_packet_version_req() {
        let command = CertificationDownlinkMacCommand::decode(&[PACKET_VERSION_CID]).unwrap();
        assert!(matches!(
            command,
            CertificationDownlinkMacCommand::PackageVersionReq
        ));
    }
    #[test]
    fn decode_echo_payload_req() {
        let buf = [0x08u8, 0x01, 0x02, 0x03];
        let command = CertificationDownlinkMacCommand::decode(&buf).unwrap();
        if let CertificationDownlinkMacCommand::EchoPayloadReq(payload) = command {
            assert_eq!(payload, &[0x01, 0x02, 0x03]);
        } else {
            assert!(false, "Wrong command type")
        }
    }
}
