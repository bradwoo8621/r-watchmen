use crate::{Encryptor, StrEncryptor};
use base64::{engine::general_purpose::STANDARD as base64, Engine};
use md5::compute;
use watchmen_base::StdR;
use watchmen_model::{FactorEncryptMethod, TopicDataValue};

pub struct Md5Encrypt {
    method: FactorEncryptMethod,
}

impl Md5Encrypt {
    pub fn new() -> Self {
        Self {
            method: FactorEncryptMethod::Md5,
        }
    }
}

impl StrEncryptor for Md5Encrypt {
    fn do_encrypt(&self, value: String) -> String {
        format!("{{MD5}}{}", base64.encode(&compute(value.as_bytes()).0))
    }
}

impl Encryptor for Md5Encrypt {
    fn method(&self) -> &FactorEncryptMethod {
        &self.method
    }

    fn accept(&self, method: &FactorEncryptMethod) -> bool {
        match method {
            FactorEncryptMethod::Md5 => true,
            _ => false,
        }
    }

    fn is_encrypted(&self, value: &TopicDataValue) -> bool {
        match value {
            TopicDataValue::Str(s) => s.starts_with("{MD5}"),
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
                if let Some(decrypted) = s.strip_prefix("{MD5}") {
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
    use crate::{EncryptorUtils, Md5Encrypt, StrEncryptor};
    use watchmen_model::TopicDataValue;

    #[test]
    fn test() {
        let encryptor = Md5Encrypt::new();
        assert_eq!(
            EncryptorUtils::get_str(encryptor.encrypt(&TopicDataValue::Str("abc".to_string()))),
            "{MD5}kAFQmDzST7DWlj99KOF/cg=="
        )
    }
}
