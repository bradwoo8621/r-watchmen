use bigdecimal::BigDecimal;
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Debug;
use watchmen_base::serde::{naive_date, naive_datetime, naive_time};
use watchmen_model_marco::VariousValueTypes;

/// the instance data id of topic
pub type TopicDataId = String;

/// TODO date-related variants might be in front of str? serde is try to parse base on the variants order
#[derive(Deserialize, Serialize, Clone, Debug, VariousValueTypes)]
#[serde(untagged)]
pub enum TopicDataValue {
    #[serde(with = "naive_datetime")]
    DateTime(NaiveDateTime),
    #[serde(with = "naive_date")]
    Date(NaiveDate),
    #[serde(with = "naive_time")]
    Time(NaiveTime),
    Str(String),
    Num(BigDecimal),
    Bool(bool),
    Map(HashMap<String, TopicDataValue>),
    Vec(Vec<TopicDataValue>),
    None,
}

pub type TopicData = HashMap<String, TopicDataValue>;
