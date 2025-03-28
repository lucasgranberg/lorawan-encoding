use crate::types::MIC;

#[cfg(feature = "soft-crypto")]
pub mod soft;

pub enum Key {
    Network,
    Application,
}
/// Trait for implementations of AES128 encryption.
pub trait Crypto {
    type Encrypter: Encrypter;
    type Mac: Mac;
    fn get_encrypter(&mut self, key: Key) -> Self::Encrypter;
    fn get_mac(&mut self) -> Self::Mac;
}

pub trait Encrypter {
    fn encrypt_block(&mut self, block: &mut [u8]);
}
pub trait Mac {
    fn calculate_mic(&mut self, data: &[&[u8]]) -> MIC;
}
