use crate::config::Config;

use rand::Rng;
use serde::{Deserialize, Serialize};
use short_crypt::ShortCrypt;
use std::fmt::{self, Display};
use std::time::{SystemTime, UNIX_EPOCH};

use nuclear::core::{async_trait, Injector, Instance, Result};

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Key(pub Box<str>);

impl Display for Key {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub struct Crypto(pub ShortCrypt);

impl Crypto {
    pub fn generate(&self) -> Key {
        let mut rng = rand::thread_rng();
        let x1: u32 = rng.gen::<u32>();

        let x2 = match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(d) => d.subsec_nanos(),
            Err(_) => rng.gen::<u32>(),
        };

        let x3: u32 = rng.gen::<u32>().wrapping_pow(1926_0817);

        let id = x1 ^ x2 ^ x3;
        let bytes = id.to_ne_bytes();

        let mut s: String = self.0.encrypt_to_qr_code_alphanumeric(&bytes);
        s.make_ascii_lowercase();
        Key(s.into())
    }

    pub fn validate(&self, input: &str) -> Option<Key> {
        let mut s: Box<str> = input.to_ascii_uppercase().into();

        let v = self.0.decrypt_qr_code_alphanumeric(&s).ok()?;

        if v.len() != 4 {
            return None;
        }

        s.make_ascii_lowercase();
        Some(Key(s))
    }
}

#[async_trait]
impl Instance for Crypto {
    async fn resolve(injector: &Injector) -> Result<Self> {
        let config = injector.try_inject_ref::<Config>()?;
        let crypt = short_crypt::ShortCrypt::new(&config.security.crypto_key);
        Ok(Self(crypt))
    }

    fn deps() -> Vec<std::any::TypeId> {
        nuclear::declare_deps![Config]
    }
}
