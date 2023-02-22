use bytestring::ByteString;
use rand::Rng;
use serde::{Deserialize, Serialize};
use short_crypt::ShortCrypt;

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Key(ByteString);

impl Key {
    #[inline(always)]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

pub struct Crypto(ShortCrypt);

impl Crypto {
    pub fn new(secret_key: &str) -> Self {
        Self(ShortCrypt::new(secret_key))
    }

    pub fn generate(&self) -> Key {
        let rand_bytes: [u8; 4] = rand::thread_rng().gen();

        let mut s: String = self.0.encrypt_to_qr_code_alphanumeric(&rand_bytes);
        s.make_ascii_lowercase();
        Key(s.into())
    }

    pub fn validate(&self, input: &str) -> Option<Key> {
        // 忽略输入的大小写
        let mut s: Box<str> = input.to_ascii_uppercase().into();

        let v = self.0.decrypt_qr_code_alphanumeric(&s).ok()?;
        if v.len() != 4 {
            return None;
        }

        // 统一表示为小写
        s.make_ascii_lowercase();
        Some(Key(s.into()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let secret_key = "asdf";
        let crypto = Crypto::new(secret_key);
        let k1 = crypto.generate();
        println!("k1 = {k1:?}");

        let k2 = crypto.validate(k1.as_str()).unwrap();
        assert_eq!(k1, k2);

        let k3 = crypto.validate(&k1.as_str().to_ascii_uppercase()).unwrap();
        assert_eq!(k1, k3);
    }
}
