const LINK_CHECK_CID: u8 = 0x02;
const LINK_ADR_CID: u8 = 0x03;
const DUTY_CYCLE_CID: u8 = 0x04;
const RX_PARAM_SETUP_CID: u8 = 0x05;
const DEV_STATUS_CID: u8 = 0x06;
const NEW_CHANNEL_CID: u8 = 0x07;
const RX_TIMING_SETUP_CID: u8 = 0x08;
const TX_PARAM_SETUP_CID: u8 = 0x09;
const DI_CHANNEL_CID: u8 = 0x0A;
const DEVICE_TIME_CID: u8 = 0x0D;

pub mod downlink;
pub mod uplink;

#[cfg(feature = "certification")]
pub mod certification;
