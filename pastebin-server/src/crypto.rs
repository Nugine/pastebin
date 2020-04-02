use rand::Rng;
use serde::{Deserialize, Serialize};
use short_crypt::ShortCrypt;
use std::fmt::{self, Display};
use std::time::{SystemTime, UNIX_EPOCH};
use nuclear::re_exports::async_trait;
use nuclear::{Injector, Provider};
use crate::config::Config;

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Key(pub Box<str>);

impl Display for Key {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Key {
    pub fn generate(crypt: &ShortCrypt) -> Key {
        let mut rng = rand::thread_rng();
        let x1: u32 = rng.gen::<u32>();

        let x2 = match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(d) => d.subsec_nanos(),
            Err(_) => rng.gen::<u32>(),
        };

        let x3: u32 = rng.gen::<u32>().wrapping_pow(1926_0817);

        let id = x1 ^ x2 ^ x3;
        let bytes = id.to_ne_bytes();

        let mut s: String = crypt.encrypt_to_qr_code_alphanumeric(&bytes);
        s.make_ascii_lowercase();
        Key(s.into())
    }

    pub fn validate(crypt: &ShortCrypt, input: &str) -> Option<Key> {
        let mut s: Box<str> = input.to_ascii_uppercase().into();

        let v = crypt.decrypt_qr_code_alphanumeric(&s).ok()?;

        if v.len() != 4 {
            return None;
        }

        s.make_ascii_lowercase();
        Some(Key(s))
    }
}

pub struct CryptoProvider;

#[async_trait]
impl Provider for CryptoProvider {
    async fn resolve(&self,injector: &mut Injector)->Option<Result<(),Box<dyn std::error::Error + Send + Sync>>>{
        let config = injector.inject_ref::<Config>()?;
        let crypt = short_crypt::ShortCrypt::new(&config.security.crypto_key);
        injector.provide(crypt);
        Some(Ok(()))
    }
}
