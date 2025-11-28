use watchmen_model::StdErrorCode;

pub enum RuntimeModelKernelErrorCode {
    SnowflakeNodeIdTooBig,
    CannotGetIdGenerator,
    CannotSetIdGenerator,

    TopicIdMissed,
    TopicIdIsBlank,
    TopicNameMissed,
    TopicNameIsBlank,
    TopicTypeMissed,
    TopicKindMissed,
    TopicTenantMissed,
    TopicTenantIsBlank,
    TopicFactorMissed,

    FactorIdMissed,
    FactorIdIsBlank,
    FactorNameMissed,
    FactorNameIsBlank,
    FactorTypeMissed,

    PipelineIdMissed,
    PipelineIdIsBlank,
    PipelineTypeMissed,
    PipelineStageMissed,
    PipelineUnitMissed,
    PipelineActionMissed,

    ConditionMissed,

    ActionSourceMissed,
    ActionVariableNameMissed,
}

impl StdErrorCode for RuntimeModelKernelErrorCode {
    fn code(&self) -> &'static str {
        match self {
            RuntimeModelKernelErrorCode::SnowflakeNodeIdTooBig => "RTMK-00001",
            RuntimeModelKernelErrorCode::CannotGetIdGenerator => "RTMK-00002",
            RuntimeModelKernelErrorCode::CannotSetIdGenerator => "RTMK-00003",

            RuntimeModelKernelErrorCode::TopicIdMissed => "RTMK-00100",
            RuntimeModelKernelErrorCode::TopicIdIsBlank => "RTMK-00101",
            RuntimeModelKernelErrorCode::TopicNameMissed => "RTMK-00102",
            RuntimeModelKernelErrorCode::TopicNameIsBlank => "RTMK-00103",
            RuntimeModelKernelErrorCode::TopicTypeMissed => "RTMK-00104",
            RuntimeModelKernelErrorCode::TopicKindMissed => "RTMK-00105",
            RuntimeModelKernelErrorCode::TopicTenantMissed => "RTMK-00106",
            RuntimeModelKernelErrorCode::TopicTenantIsBlank => "RTMK-00107",
            RuntimeModelKernelErrorCode::TopicFactorMissed => "RTMK-00108",
            RuntimeModelKernelErrorCode::FactorIdMissed => "RTMK-00109",
            RuntimeModelKernelErrorCode::FactorIdIsBlank => "RTMK-00110",
            RuntimeModelKernelErrorCode::FactorNameMissed => "RTMK-00111",
            RuntimeModelKernelErrorCode::FactorNameIsBlank => "RTMK-00112",
            RuntimeModelKernelErrorCode::FactorTypeMissed => "RTMK-00113",

            RuntimeModelKernelErrorCode::PipelineIdMissed => "RTMK-00200",
            RuntimeModelKernelErrorCode::PipelineIdIsBlank => "RTMK-00201",
            RuntimeModelKernelErrorCode::PipelineTypeMissed => "RTMK-00202",
            RuntimeModelKernelErrorCode::PipelineStageMissed => "RTMK-00203",
            RuntimeModelKernelErrorCode::PipelineUnitMissed => "RTMK-00204",
            RuntimeModelKernelErrorCode::PipelineActionMissed => "RTMK-00205",

            RuntimeModelKernelErrorCode::ConditionMissed => "RTMK-00206",

            RuntimeModelKernelErrorCode::ActionSourceMissed => "RTMK-00207",
            RuntimeModelKernelErrorCode::ActionVariableNameMissed => "RTMK-00208",
        }
    }
}
