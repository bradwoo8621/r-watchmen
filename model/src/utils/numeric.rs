use crate::{StdErr, StdErrCode, StdErrorCode};
use bigdecimal::BigDecimal;
use std::str::FromStr;

pub trait NumericUtils {
    fn is_numeric(&self) -> bool;
    fn to_decimal(&self) -> Result<BigDecimal, StdErr>;
}

impl NumericUtils for String {
    /// radix 10
    fn is_numeric(&self) -> bool {
        self.chars().all(|c| c.is_digit(10))
    }

    fn to_decimal(&self) -> Result<BigDecimal, StdErr> {
        if let Ok(v) = BigDecimal::from_str(&self) {
            Ok(v)
        } else {
            StdErr::of(
                StdErrCode::DecimalParse.code(),
                format!("Cannot parse '{}' to decimal", self),
            )
        }
    }
}
