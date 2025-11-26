use bigdecimal::{BigDecimal, Zero};
use std::str::FromStr;

pub trait BooleanUtils {
    fn to_bool(&self) -> bool;
}

impl BooleanUtils for String {
    fn to_bool(&self) -> bool {
        let val = &self.to_lowercase();
        let val = val.as_str();
        match val {
            "true" | "t" | "yes" | "y" => true,
            "false" | "f" | "no" | "n" => false,
            _ => {
                if let Ok(v) = BigDecimal::from_str(val) {
                    v != BigDecimal::zero()
                } else {
                    false
                }
            }
        }
    }
}

impl BooleanUtils for &str {
    fn to_bool(&self) -> bool {
        self.to_string().to_bool()
    }
}
