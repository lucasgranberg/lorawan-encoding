use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout};
#[allow(dead_code)]
#[derive(IntoBytes, FromBytes, KnownLayout, Immutable)]
pub struct DevEui([u8; 8]);

#[allow(dead_code)]
#[derive(IntoBytes, FromBytes, KnownLayout, Immutable)]
pub struct JoinEui([u8; 8]);

#[allow(dead_code)]
#[derive(IntoBytes, FromBytes, KnownLayout, Immutable)]
pub struct DevNonce([u8; 2]);
