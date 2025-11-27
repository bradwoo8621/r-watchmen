use watchmen_model::StdErrorCode;

pub enum RuntimeModelKernelErrorCode {
    SnowflakeNodeIdTooBig,
}

impl StdErrorCode for RuntimeModelKernelErrorCode {
    fn code(&self) -> &'static str {
        match self {
            RuntimeModelKernelErrorCode::SnowflakeNodeIdTooBig => "RTMK-00001",
        }
    }
}
