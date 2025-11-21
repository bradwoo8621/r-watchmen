use crate::serde::option_naive_datetime;
use crate::{
    Auditable, BaseDataModel, BucketId, FactorOrObjectiveFactorIdOrSubjectDatasetColumnId,
    IndicatorId, OptimisticLock, Storable, TenantBasedTuple, TenantId, Tuple, UserGroupId, UserId,
};
use serde::{Deserialize, Serialize};
use watchmen_model_marco::{adapt_model, Display, Serde};

pub type ObjectiveTargetId = String;

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

#[derive(Display, Serde)]
pub enum ObjectiveTimeframeKind {
    None,
    Year,
    HalfYear,
    Quarter,
    Month,
    WeekOfYear,
    DayOfMonth,
    DayOfWeek,
    LastNYears,
    LastNMonths,
    LastNWeeks,
    LastNDays,
}

#[derive(Display, Serde)]
pub enum ObjectiveTimeframeTill {
    Now,
    LastCompleteCycle,
    Specified,
}

#[adapt_model(storable)]
pub struct ObjectiveTimeframe {
    /// is target in time frame, normally is
    pub kind: Option<ObjectiveTimeframeKind>,
    /// only available if kind is LAST_N-* types, should be a positive value
    pub last_n: Option<String>,
    /// time frame is cut off till when
    pub till: Option<ObjectiveTimeframeTill>,
    /// specify the till time when [till] is [ObjectiveTimeframeTill::Specified]
    pub specified_till: Option<String>,
}

#[derive(Display, Serde)]
pub enum ObjectiveTargetBetterSide {
    Less,
    More,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum ObjectiveTargetAsIs {
    Parameter(ComputedObjectiveParameter),
    Factor(ObjectiveFactorId),
}

#[adapt_model(storable)]
pub struct ObjectiveTarget {
    pub uuid: Option<ObjectiveTargetId>,
    pub name: Option<String>,
    /// to be value, should be a numeric value, a percentage value
    pub tobe: Option<String>,
    /// as is formula
    pub asis: Option<ObjectiveTargetAsIs>,
    /// which side is better, with computed as is value vs to be value.
    pub better_side: Option<ObjectiveTargetBetterSide>,
    /// this July vs this June if time frame is on month, month-on-month
    pub ask_previous_cycle: Option<bool>,
    /// this July vs last July if time frame is on month, year-on-year
    pub ask_chain_cycle: Option<bool>,
}

#[derive(Display, Serde)]
pub enum ObjectiveVariableKind {
    #[display = "value"]
    SingleValue,
    Bucket,
    Range,
}

#[adapt_model(storable)]
pub struct ObjectiveVariable {
    pub name: Option<String>,
    pub kind: Option<ObjectiveVariableKind>,
}

#[adapt_model(storable)]
pub struct ObjectiveVariableOnValue {
    //(ObjectiveVariable):
    pub kind: Option<ObjectiveVariableKind>, // ObjectiveVariableKind.SINGLE_VALUE
    pub value: Option<String>,
}

#[adapt_model(storable)]
pub struct ObjectiveVariableOnBucket {
    //(ObjectiveVariable):
    pub kind: Option<ObjectiveVariableKind>, // ObjectiveVariableKind.BUCKET
    pub bucket_id: Option<BucketId>,
    pub segment_name: Option<String>,
}

#[adapt_model(storable)]
pub struct ObjectiveVariableOnRange {
    //(ObjectiveVariable):
    pub kind: Option<ObjectiveVariableKind>, // ObjectiveVariableKind.RANGE
    pub min: Option<String>,
    pub include_min: Option<bool>,
    pub max: Option<String>,
    pub include_max: Option<bool>,
}

#[derive(Display, Serde)]
pub enum ObjectiveFactorKind {
    Indicator,
    Computed,
}

pub type ObjectiveFactorId = String;
pub type ObjectiveFactorName = String;

#[adapt_model(storable)]
pub struct ObjectiveFactorOnIndicator {
    pub uuid: Option<ObjectiveFactorId>,
    pub kind: Option<ObjectiveFactorKind>,
    pub name: Option<ObjectiveFactorName>,
    pub formula: Option<ComputedObjectiveParameter>,
    pub indicator_id: Option<IndicatorId>,
    pub conditional: Option<bool>,
    /// objective variables are available in constant value
    pub filter: Option<ObjectiveParameterJoint>,
}

impl ObjectiveFactorOnIndicator {
    pub fn init() -> Self {
        ObjectiveFactorOnIndicator::new().kind(ObjectiveFactorKind::Indicator)
    }

    pub fn to_factor(self) -> ObjectiveFactor {
        ObjectiveFactor::Indicator(self)
    }
}

#[adapt_model(storable)]
pub struct ObjectiveFactorOnComputation {
    pub uuid: Option<ObjectiveFactorId>,
    pub kind: Option<ObjectiveFactorKind>,
    pub name: Option<ObjectiveFactorName>,
    pub formula: Option<ComputedObjectiveParameter>,
}

impl ObjectiveFactorOnComputation {
    pub fn init() -> Self {
        ObjectiveFactorOnComputation::new().kind(ObjectiveFactorKind::Computed)
    }

    pub fn to_factor(self) -> ObjectiveFactor {
        ObjectiveFactor::Computed(self)
    }
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum ObjectiveFactor {
    Indicator(ObjectiveFactorOnIndicator),
    Computed(ObjectiveFactorOnComputation),
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
