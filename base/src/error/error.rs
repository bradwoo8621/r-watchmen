use crate::{ErrorCode, StdErrCode, StdR};
use serde::Serialize;
use std::fmt::{Display, Formatter};
use std::panic::Location;

#[derive(Serialize, Debug)]
#[serde(untagged)]
pub enum StdErrDetail {
    Str(String),
    Sub(Vec<StdErr>),
}

impl Display for StdErrDetail {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Str(s) => {
                write!(f, "{}", s)
            }
            Self::Sub(vec) => {
                write!(
                    f,
                    "{}",
                    vec.iter()
                        .map(|se| format!("{}", se))
                        .collect::<Vec<String>>()
                        .join(", ")
                )
            }
        }
    }
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

    // location
    pub filename: String,
    pub line: u32,
    pub column: u32,
}

impl Display for StdErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "StdErr[code={}, details={}] at [file={}, line={}, column={}]",
            self.code,
            self.details
                .as_ref()
                .map(|d| format!("{}", d))
                .unwrap_or(String::new()),
            self.filename,
            self.line,
            self.column
        )
    }
}

impl StdErr {
    #[track_caller]
    pub fn of<R, M>(code: &'static str, msg: M) -> Result<R, Self>
    where
        M: Into<String>,
    {
        Self::of_with_location(code, msg, Location::caller())
    }

    pub fn of_with_location<R, M>(
        code: &'static str,
        msg: M,
        location: &Location,
    ) -> Result<R, Self>
    where
        M: Into<String>,
    {
        Err(Self {
            code,
            details: Some(StdErrDetail::Str(msg.into())),
            filename: location.file().to_string(),
            line: location.line(),
            column: location.column(),
        })
    }

    #[track_caller]
    pub fn code_only<R>(code: &'static str) -> Result<R, Self> {
        Self::code_only_with_location(code, Location::caller())
    }

    pub fn code_only_with_location<R>(code: &'static str, location: &Location) -> Result<R, Self> {
        Err(Self {
            code,
            details: None,
            filename: location.file().to_string(),
            line: location.line(),
            column: location.column(),
        })
    }

    /// message only
    #[track_caller]
    pub fn unknown<R, M>(msg: M) -> Result<R, Self>
    where
        M: Into<String>,
    {
        Self::unknown_with_location(msg, Location::caller())
    }

    /// message only
    pub fn unknown_with_location<R, M>(msg: M, location: &Location) -> Result<R, Self>
    where
        M: Into<String>,
    {
        Err(Self {
            code: StdErrCode::Unknown.code(),
            details: Some(StdErrDetail::Str(msg.into())),
            filename: location.file().to_string(),
            line: location.line(),
            column: location.column(),
        })
    }

    #[track_caller]
    pub fn accumulate<R>(details: Vec<StdErr>) -> StdR<R> {
        Self::accumulate_with_location(details, Location::caller())
    }

    pub fn accumulate_with_location<R>(details: Vec<StdErr>, location: &Location) -> StdR<R> {
        Err(Self {
            code: StdErrCode::Multiple.code(),
            details: Some(StdErrDetail::Sub(details)),
            filename: location.file().to_string(),
            line: location.line(),
            column: location.column(),
        })
    }
}
