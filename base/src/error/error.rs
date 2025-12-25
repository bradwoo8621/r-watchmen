use crate::{StdErrCode, ErrorCode, StdR};
use serde::Serialize;

#[derive(Serialize, Debug)]
#[serde(untagged)]
pub enum StdErrDetail {
    Str(String),
    Sub(Vec<StdErr>),
}

/// In theory, errors support an infinite number of levels.
/// However, in normal use, you should try to keep it to two levels.
///
/// Convert other types of exceptions to this exception to enable the use of the `?` syntactic sugar.
#[derive(Serialize, Debug)]
pub struct StdErr {
    /// code must be [XXXX-99999], each module has its own code prefix [XXXX]
    pub code: &'static str,
    pub details: Option<StdErrDetail>,
}

impl StdErr {
    pub fn of<R, M>(code: &'static str, msg: M) -> Result<R, Self>
    where
        M: Into<String>,
    {
        Err(Self {
            code,
            details: Some(StdErrDetail::Str(msg.into())),
        })
    }

    pub fn code_only<R>(code: &'static str) -> Result<R, Self> {
        Err(Self {
            code,
            details: None,
        })
    }

    /// message only
    pub fn unknown<R, M>(msg: M) -> Result<R, Self>
    where
        M: Into<String>,
    {
        Err(Self {
            code: StdErrCode::Unknown.code(),
            details: Some(StdErrDetail::Str(msg.into())),
        })
    }

    pub fn accumulate<R>(details: Vec<StdErr>) -> StdR<R> {
        Err(Self {
            code: StdErrCode::Multiple.code(),
            details: Some(StdErrDetail::Sub(details)),
        })
    }
}
