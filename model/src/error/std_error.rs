use serde::{Deserialize, Serialize};

pub trait StdErrorCode {
    fn code(&self) -> &'static str;
}

pub enum StdErrCode {
    DecimalParse,
    Unknown,
}

impl StdErrorCode for StdErrCode {
    fn code(&self) -> &'static str {
        match self {
            StdErrCode::DecimalParse => "STDE-00001",
            StdErrCode::Unknown => "STDE-99999",
        }
    }
}

/// Convert other types of exceptions to this exception to enable the use of the `?` syntactic sugar.
#[derive(Serialize, Deserialize)]
pub struct StdErr {
    /// code must be [XXXX-99999], each module has its own code prefix [XXXX]
    code: &'static str,
    msg: Option<String>,
}

impl StdErr {
    pub fn of<R, M>(code: &'static str, msg: M) -> Result<R, Self>
    where
        M: Into<String>,
    {
        Err(StdErr {
            code,
            msg: Some(msg.into()),
        })
    }

    /// code only
    pub fn co<R>(code: &'static str) -> Result<R, Self> {
        Err(StdErr { code, msg: None })
    }

    /// message only
    pub fn mo<R, M>(msg: M) -> Result<R, Self>
    where
        M: Into<String>,
    {
        Err(StdErr {
            code: StdErrCode::Unknown.code(),
            msg: Some(msg.into()),
        })
    }
}
