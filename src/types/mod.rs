use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout};
#[allow(dead_code)]
#[derive(Clone, Copy, IntoBytes, FromBytes, KnownLayout, Immutable)]
pub struct DevEui([u8; 8]);

#[allow(dead_code)]
#[derive(Clone, Copy, IntoBytes, FromBytes, KnownLayout, Immutable)]
pub struct JoinEui([u8; 8]);

#[allow(dead_code)]
#[derive(Clone, Copy, IntoBytes, FromBytes, KnownLayout, Immutable)]
pub struct DevNonce([u8; 2]);

#[allow(dead_code)]
#[derive(Clone, Copy, IntoBytes, FromBytes, KnownLayout, Immutable)]
pub struct DevAddr([u8; 4]);

#[allow(dead_code)]
#[derive(PartialEq, Eq, Clone, Copy, IntoBytes, FromBytes, KnownLayout, Immutable)]
pub struct MIC([u8; 4]);

#[allow(dead_code)]
#[derive(Clone, Copy, IntoBytes, FromBytes, KnownLayout, Immutable)]
pub struct NwkSKey([u8; 16]);

#[allow(dead_code)]
#[derive(Clone, Copy, IntoBytes, FromBytes, KnownLayout, Immutable)]
pub struct AppSKey([u8; 16]);
