use aes::cipher::{generic_array::GenericArray, BlockEncrypt, KeyInit};
use aes::{Aes128, Aes128Enc};
use cmac::{Cmac, Mac};
use zerocopy::{FromBytes, IntoBytes};

use crate::types::MIC;
use crate::types::{AppSKey, NwkSKey};

use super::{Crypto, Encrypter, Key};

pub struct SoftCrypto {
    nwk_s_key: NwkSKey,
    app_s_key: AppSKey,
}
impl SoftCrypto {
    pub fn new(nwk_s_key: NwkSKey, app_s_key: AppSKey) -> Self {
        Self {
            nwk_s_key,
            app_s_key,
        }
    }
}

pub struct SoftEncrypter {
    inner: Aes128Enc,
}
impl Encrypter for SoftEncrypter {
    fn encrypt_block(&mut self, block: &mut [u8]) {
        self.inner
            .encrypt_block(GenericArray::from_mut_slice(block))
    }
}
pub struct SoftMac {
    inner: Cmac<Aes128>,
}
impl super::Mac for SoftMac {
    fn calculate_mic(&mut self, data: &[&[u8]]) -> MIC {
        let mac = &mut self.inner;
        for row in data {
            mac.update(*row);
        }
        let result = mac.finalize_reset().into_bytes();
        MIC::read_from_bytes(&result.as_slice()[..4]).unwrap()
    }
}

impl Crypto for SoftCrypto {
    type Encrypter = SoftEncrypter;

    type Mac = SoftMac;

    fn get_encrypter(&mut self, key: Key) -> Self::Encrypter {
        let inner = match key {
            Key::Network => Aes128Enc::new(GenericArray::from_slice(self.nwk_s_key.as_bytes())),
            Key::Application => Aes128Enc::new(GenericArray::from_slice(self.app_s_key.as_bytes())),
        };
        Self::Encrypter { inner }
    }

    fn get_mac(&mut self) -> Self::Mac {
        Self::Mac {
            inner: <Cmac<Aes128> as cmac::Mac>::new_from_slice(self.nwk_s_key.as_bytes()).unwrap(),
        }
    }
}
