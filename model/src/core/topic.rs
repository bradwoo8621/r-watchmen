use crate::serde::{naive_date, naive_datetime, naive_time, option_naive_datetime};
use crate::{
    Auditable, BaseDataModel, DataSourceId, Factor, OptimisticLock, Storable, TenantBasedTuple,
    TenantId, Tuple, UserId,
};
use bigdecimal::BigDecimal;
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;
use std::collections::HashMap;
use watchmen_model_marco::{adapt_model, Display, Serde, StrEnum, VariousValueTypes};

#[derive(Display, Serde, Debug, StrEnum)]
pub enum TopicKind {
    System,
    Business,
    Synonym,
}

#[derive(Display, Serde, PartialEq, Debug, StrEnum)]
pub enum TopicType {
    Raw,
    Meta,
    Distinct,
    Aggregate,
    Time,
    Ratio,
}

impl TopicType {
    pub fn is_raw_topic(&self) -> bool {
        *self == TopicType::Raw
    }

    pub fn is_aggregation_topic(&self) -> bool {
        *self == TopicType::Aggregate
    }
}

pub type TopicId = String;

#[adapt_model(opt_lock, tenant_based)]
pub struct Topic {
    pub topic_id: Option<TopicId>,
    pub name: Option<String>,
    pub r#type: Option<TopicType>,
    pub kind: Option<TopicKind>,
    pub data_source_id: Option<DataSourceId>,
    pub factors: Option<Vec<Factor>>,
    pub description: Option<String>,
}

impl Topic {
    pub fn is_raw_topic(&self) -> bool {
        self.r#type
            .as_ref()
            .map(|t| t.is_raw_topic())
            .unwrap_or(false)
    }

    pub fn is_aggregation_topic(&self) -> bool {
        self.r#type
            .as_ref()
            .map(|t| t.is_aggregation_topic())
            .unwrap_or(false)
    }
}

/// the instance data id of topic
pub type TopicDataId = String;

#[derive(Deserialize, Serialize, Clone, Debug, VariousValueTypes)]
#[serde(untagged)]
pub enum TopicDataValue {
    Str(String),
    Num(BigDecimal),
    Bool(bool),
    #[serde(with = "naive_datetime")]
    DateTime(NaiveDateTime),
    #[serde(with = "naive_date")]
    Date(NaiveDate),
    #[serde(with = "naive_time")]
    Time(NaiveTime),
    Map(HashMap<String, TopicDataValue>),
    Vec(Vec<TopicDataValue>),
    None,
}

pub type TopicData = HashMap<String, TopicDataValue>;
