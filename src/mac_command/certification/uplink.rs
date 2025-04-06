use zerocopy::{Immutable, IntoBytes, KnownLayout, Unaligned};

#[repr(u8)]
pub enum CertificationUplinkMacCommand {
    /// Conveys the answer to PackageVersionReq
    PackageVersionAns(PackageVersionAns) = 0x00,
    /// Conveys the answer to EchoPayloadReq request
    EchoPayloadAns = 0x08,
    /// Conveys the answer to RxAppCntReq request
    RxAppCntAns = 0x09,
    /// Conveys the answer to DutVersionsReq request
    DutVersionsAns = 0x7F,
}

#[derive(Clone, Debug, IntoBytes, Immutable, KnownLayout, Unaligned)]
#[repr(C)]
pub struct PackageVersionAns {
    package_identifier: u8,
    package_version: u8,
}
