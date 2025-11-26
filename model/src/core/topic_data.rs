use crate::serde::{naive_date, naive_datetime, naive_time};
use bigdecimal::BigDecimal;
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use watchmen_model_marco::VariousValueTypes;

/// the instance data id of topic
pub type TopicDataId = String;

/// TODO date-related might be in front of str?
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
