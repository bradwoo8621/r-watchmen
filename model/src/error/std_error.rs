use serde::{Deserialize, Serialize};
use watchmen_model_marco::{Display, Serde};

#[derive(Display, Serde, Debug, Clone)]
pub enum StdErrCode {
    #[display = "99999"]
    Unknown,
}

/// Convert other types of exceptions to this exception to enable the use of the `?` syntactic sugar.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StdErr {
    code: StdErrCode,
    msg: String,
}

impl StdErr {
    pub fn of<R>(code: StdErrCode, msg: String) -> Result<R, Self> {
        Err(StdErr { code, msg })
    }

    /// code only
    pub fn co<R>(code: StdErrCode) -> Result<R, Self> {
        Err(StdErr {
            code,
            msg: String::from(""),
        })
    }

    /// message only
    pub fn mo<R>(msg: String) -> Result<R, Self> {
        Err(StdErr {
            code: StdErrCode::Unknown,
            msg,
        })
    }
}
