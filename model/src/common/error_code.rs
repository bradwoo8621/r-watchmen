use watchmen_base::ErrorCode;

pub enum ModelErrorCode {
    StrEnumParse,
    VariableFunctionParse,
}

impl ErrorCode for ModelErrorCode {
    fn code(&self) -> &'static str {
        match self {
            ModelErrorCode::StrEnumParse => "MDLE-00001",
            ModelErrorCode::VariableFunctionParse => "MDLE-00002",
        }
    }
}
