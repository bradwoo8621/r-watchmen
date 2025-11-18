use crate::serde::option_naive_datetime;
use crate::{
    Auditable, BaseDataModel, ConnectedSpaceId, FactorId, LastVisit, Parameter, ParameterCondition,
    ParameterJoint, Storable, TenantId, TopicId, UserBasedTuple, UserId,
};
use watchmen_model_marco::{adapt_model, Display, Serde};

#[derive(Display, Serde)]
pub enum SubjectJoinType {
    Left,
    Right,
    Inner,
}

#[adapt_model(storable)]
pub struct SubjectDatasetJoin {
    pub topic_id: Option<TopicId>,
    pub factor_id: Option<FactorId>,
    pub secondary_topic_id: Option<TopicId>,
    pub secondary_factor_id: Option<FactorId>,
    pub r#type: Option<SubjectJoinType>,
}

#[derive(Display, Serde)]
pub enum SubjectColumnArithmetic {
    None,
    #[display = "distinct_count"]
    DistinctCount,
    Count,
    Sum,
    Avg,
    Max,
    Min,
}

#[derive(Display, Serde)]
pub enum SubjectColumnAlignment {
    Left,
    Center,
    Right,
}

#[derive(Display, Serde)]
pub enum SubjectColumnFormat {
    None,
    #[display = "#,##0"]
    UseGroup,
    #[display = "#,##0.0"]
    UseGroup1,
    #[display = "#,##0.00"]
    UseGroup2,
    #[display = "#,##0.000"]
    UseGroup3,
    #[display = "#,##0.0000"]
    UseGroup4,
    #[display = "#,##0.00000"]
    UseGroup5,
    #[display = "#,##0.000000"]
    UseGroup6,
}

#[adapt_model(storable)]
pub struct SubjectDataSetColumnRenderer {
    pub alignment: Option<SubjectColumnAlignment>,
    pub format: Option<SubjectColumnFormat>,
    pub highlight_negative: Option<bool>,
}

pub type SubjectDatasetColumnId = String;

#[adapt_model(storable)]
pub struct SubjectDatasetColumn {
    pub column_id: Option<SubjectDatasetColumnId>,
    pub parameter: Option<Parameter>,
    pub alias: Option<String>,
    pub arithmetic: Option<SubjectColumnArithmetic>,
    pub renderer: Option<SubjectDataSetColumnRenderer>,
    /// recalculated column based on other columns
    /// source of this column must be referred to other column, via columnId or alias
    /// if column is declared as recalculate, arithmetic will be ignored
    /// and refer to another recalculated column is not allowed
    pub recalculate: Option<bool>,
}

#[adapt_model(storable)]
pub struct SubjectDataset {
    pub columns: Option<Vec<SubjectDatasetColumn>>,
    pub joins: Option<Vec<SubjectDatasetJoin>>,
    pub filters: Option<ParameterJoint>,
}

pub type SubjectId = String;

#[adapt_model(user_based, audit, last_visit)]
pub struct Subject {
    pub subject_id: Option<SubjectId>,
    pub name: Option<String>,
    pub connect_id: Option<ConnectedSpaceId>,
    pub auto_refresh_interval: Option<i32>,
    pub dataset: Option<SubjectDataset>,
}

#[derive(Display, Serde)]
pub enum SubjectDatasetCriteriaIndicatorArithmetic {
    None,
    #[display = "distinct_count"]
    DistinctCount,
    Count,
    Sum,
    Avg,
    Max,
    Min,
}

#[adapt_model(storable)]
pub struct SubjectDatasetCriteriaIndicator {
    pub name: Option<String>,
    pub column_id: Option<SubjectDatasetColumnId>,
    pub arithmetic: Option<SubjectDatasetCriteriaIndicatorArithmetic>,
    pub alias: Option<String>,
}

#[adapt_model(storable)]
pub struct SubjectDatasetCriteria {
    ///(Pageable):
    /// use one of subject id or name
    pub subject_id: Option<SubjectId>,
    pub subject_name: Option<String>,
    pub indicators: Option<Vec<SubjectDatasetCriteriaIndicator>>,
    pub conditions: Option<Vec<ParameterCondition>>,
}
