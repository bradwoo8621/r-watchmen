use crate::{Encryptor, StrEncryptor};
use hex::encode as hex_encode;
use sha2::{Digest, Sha256};
use watchmen_base::StdR;
use watchmen_model::{FactorEncryptMethod, TopicDataValue};

pub struct Sha256Encrypt {
    method: FactorEncryptMethod,
}

impl Sha256Encrypt {
    pub fn new() -> Self {
        Self {
            method: FactorEncryptMethod::Sha256,
        }
    }
}

impl StrEncryptor for Sha256Encrypt {
    fn do_encrypt(&self, value: String) -> String {
        let mut sha256 = Sha256::new();
        sha256.update(value.as_bytes());
        let value = sha256.finalize();
        format!("{{SHA256}}{}", hex_encode(value))
    }
}

impl Encryptor for Sha256Encrypt {
    fn method(&self) -> &FactorEncryptMethod {
        &self.method
    }

    fn accept(&self, method: &FactorEncryptMethod) -> bool {
        match method {
            FactorEncryptMethod::Sha256 => true,
            _ => false,
        }
    }

    fn is_encrypted(&self, value: &TopicDataValue) -> bool {
        match value {
            TopicDataValue::Str(s) => s.starts_with("{SHA256}"),
            _ => false,
        }
    }

    fn encrypt(&self, value: &TopicDataValue) -> StdR<Option<TopicDataValue>> {
        StrEncryptor::encrypt(self, value)
    }

    ///  md5 cannot be decrypted, remove prefix only
    fn decrypt(&self, value: &TopicDataValue) -> StdR<Option<TopicDataValue>> {
        match value {
            TopicDataValue::Str(s) => {
                if let Some(decrypted) = s.strip_prefix("{SHA256}") {
                    Ok(Some(TopicDataValue::Str(decrypted.to_string())))
                } else {
                    Ok(None)
                }
            }
            _ => Ok(None),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{EncryptorUtils, Sha256Encrypt, StrEncryptor};
    use watchmen_model::TopicDataValue;

    #[test]
    fn test() {
        let encryptor = Sha256Encrypt::new();
        assert_eq!(
            EncryptorUtils::get_str(encryptor.encrypt(&TopicDataValue::Str("abc".to_string()))),
            "{SHA256}ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        )
    }
}
