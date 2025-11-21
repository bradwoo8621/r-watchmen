use crate::serde::option_naive_datetime;
use crate::{
    Auditable, BaseDataModel, BucketId, FactorOrObjectiveFactorIdOrSubjectDatasetColumnId,
    ObjectiveFactor, ObjectiveTarget, ObjectiveTimeframe, ObjectiveVariable,
    OptimisticLock, Storable, TenantBasedTuple, TenantId, Tuple, UserGroupId, UserId,
};
use serde::{Deserialize, Serialize};
use watchmen_model_marco::{adapt_model, Display, Serde};

#[derive(Display, Serde)]
pub enum ObjectiveParameterType {
    Refer,
    Constant,
    Computed,
    /// only available on factor indicator filter
    Bucket,
    /// only available on factor indicator filter
    #[display = "time-frame"]
    Timeframe,
}

/// it's a multiple purposes object.
///	when it is used in factor/target formula, {@link #uuid} should refer to another objective factor.
///	and when it is used in factor filter, {@link #uuid} should refer to factor from topic or column from subject dataset.
#[adapt_model(storable)]
pub struct ReferObjectiveParameter {
    pub kind: Option<ObjectiveParameterType>,
    pub uuid: Option<FactorOrObjectiveFactorIdOrSubjectDatasetColumnId>,
}

impl ReferObjectiveParameter {
    pub fn init() -> Self {
        ReferObjectiveParameter::new().kind(ObjectiveParameterType::Refer)
    }

    pub fn to_parameter(self) -> ObjectiveParameter {
        ObjectiveParameter::Refer(self)
    }
}

#[adapt_model(storable)]
pub struct ConstantObjectiveParameter {
    pub kind: Option<ObjectiveParameterType>,
    pub value: Option<String>,
}

impl ConstantObjectiveParameter {
    pub fn init() -> Self {
        ConstantObjectiveParameter::new().kind(ObjectiveParameterType::Constant)
    }

    pub fn to_parameter(self) -> ObjectiveParameter {
        ObjectiveParameter::Constant(self)
    }
}

#[derive(Display, Serde)]
pub enum ObjectiveFormulaOperator {
    None,
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulus,
    YearOf,
    HalfYearOf,
    QuarterOf,
    MonthOf,
    WeekOfYear,
    WeekOfMonth,
    DayOfMonth,
    DayOfWeek,
    Round,
    Floor,
    Ceil,
    Abs,
    Max,
    Min,
    Interpolate,

    CaseThen,
}

#[adapt_model(storable)]
pub struct ComputedObjectiveParameter {
    pub kind: Option<ObjectiveParameterType>,
    pub operator: Option<ObjectiveFormulaOperator>,
    pub parameters: Option<Vec<ObjectiveParameter>>,
}

impl ComputedObjectiveParameter {
    pub fn init() -> Self {
        ComputedObjectiveParameter::new().kind(ObjectiveParameterType::Computed)
    }

    pub fn to_parameter(self) -> ObjectiveParameter {
        ObjectiveParameter::Computed(self)
    }
}

#[adapt_model(storable)]
pub struct BucketObjectiveParameter {
    pub kind: Option<ObjectiveParameterType>,
    pub bucket_id: Option<BucketId>,
    pub segment_name: Option<String>,
}

impl BucketObjectiveParameter {
    pub fn init() -> Self {
        BucketObjectiveParameter::new().kind(ObjectiveParameterType::Bucket)
    }

    pub fn to_parameter(self) -> ObjectiveParameter {
        ObjectiveParameter::Bucket(self)
    }
}

#[adapt_model(storable)]
pub struct TimeframeObjectiveParameter {
    pub kind: Option<ObjectiveParameterType>,
    // TODO
    // pub conditional: Option<bool>,
    // pub on: Option<ObjectiveParameterJoint>,
}

impl TimeframeObjectiveParameter {
    pub fn init() -> Self {
        TimeframeObjectiveParameter::new().kind(ObjectiveParameterType::Timeframe)
    }

    pub fn to_parameter(self) -> ObjectiveParameter {
        ObjectiveParameter::Timeframe(self)
    }
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum ObjectiveParameter {
    Refer(ReferObjectiveParameter),
    Constant(ConstantObjectiveParameter),
    Computed(ComputedObjectiveParameter),
    Bucket(BucketObjectiveParameter),
    Timeframe(TimeframeObjectiveParameter),
}

#[adapt_model(storable)]
pub struct ObjectiveParameterCondition {}

#[derive(Display, Serde)]
pub enum ObjectiveParameterExpressionOperator {
    Empty,
    NotEmpty,
    Equals,
    NotEquals,
    Less,
    LessEquals,
    More,
    MoreEquals,
    In,
    NotIn,
}

#[adapt_model(storable)]
pub struct ObjectiveParameterExpression {
    //(ObjectiveParameterCondition):
    pub left: Option<ObjectiveParameter>,
    pub operator: Option<ObjectiveParameterExpressionOperator>, // ObjectiveParameterExpressionOperator.EQUALS
    pub right: Option<ObjectiveParameter>,
}

#[derive(Display, Serde)]
pub enum ObjectiveParameterJointType {
    And,
    Or,
}

#[adapt_model(storable)]
pub struct ObjectiveParameterJoint {
    // (ObjectiveParameterCondition):
    pub conj: Option<ObjectiveParameterJointType>, // ObjectiveParameterJointType.AND
    pub filters: Option<Vec<ObjectiveParameterCondition>>,
}

pub type ObjectiveId = String;

#[adapt_model(opt_lock, tenant_based)]
pub struct Objective {
    pub objective_id: Option<ObjectiveId>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub time_frame: Option<ObjectiveTimeframe>,
    pub targets: Option<Vec<ObjectiveTarget>>,
    pub variables: Option<Vec<ObjectiveVariable>>,
    pub factors: Option<Vec<ObjectiveFactor>>,
    pub group_ids: Option<Vec<UserGroupId>>,
}
