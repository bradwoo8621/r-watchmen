use crate::{ArcTopicData, ArcTopicDataValue};
use bigdecimal::{BigDecimal, FromPrimitive};
use std::ops::Deref;
use std::sync::{Arc, LazyLock};
use watchmen_model::{StdErrCode, StdErrorCode, StdR, VariablePredefineFunctions};

pub enum TopicDataProperty {
    /// 0. property name,
    /// 1. is array or not
    Str((String, bool)),
    /// 0. property name,
    /// 1. names split by [.],
    /// 2. is array or not
    Vec((String, Vec<String>, bool)),
}

impl TopicDataProperty {
    ///
    pub fn of(property: &String, array: bool) -> Self {
        if property.contains('.') {
            TopicDataProperty::Vec((
                property.clone(),
                property.split('.').map(|s| s.to_string()).collect(),
                array,
            ))
        } else {
            TopicDataProperty::Str((property.clone(), array))
        }
    }
}

pub trait TopicDataUtils {
    fn value_of(&self, property: &TopicDataProperty) -> StdR<Arc<ArcTopicDataValue>>;
}

static ARC__TOPIC_VALUE__NONE: LazyLock<Arc<ArcTopicDataValue>, fn() -> Arc<ArcTopicDataValue>> =
    LazyLock::new(|| Arc::new(ArcTopicDataValue::None));
static ARC__TOPIC_VALUE__EMPTY_VEC: LazyLock<
    Arc<ArcTopicDataValue>,
    fn() -> Arc<ArcTopicDataValue>,
> = LazyLock::new(|| Arc::new(ArcTopicDataValue::Vec(vec![].into())));

impl TopicDataUtils for ArcTopicData {
    fn value_of(&self, property: &TopicDataProperty) -> StdR<Arc<ArcTopicDataValue>> {
        match property {
            TopicDataProperty::Str((name, _)) => {
                // use none if name not exists, never mind the array or not
                let value = self.get(name).clone();
                if value.is_some() {
                    Ok(value.unwrap().clone())
                } else {
                    Ok(ARC__TOPIC_VALUE__NONE.clone())
                }
            }
            TopicDataProperty::Vec((name, names, array)) => {
                let data = self.get(&names[0]);
                if data.is_none() {
                    return if *array {
                        Ok(ARC__TOPIC_VALUE__EMPTY_VEC.clone())
                    } else {
                        Ok(ARC__TOPIC_VALUE__NONE.clone())
                    };
                }

                let mut data = data.unwrap();
                let mut remain_count = names.len() - 1;
                let mut current_index = 1;
                while current_index <= remain_count {
                    let current_name = &names[current_index];
                    if let Some(func) = VariablePredefineFunctions::try_parse(current_name) {
                        match func {
                            VariablePredefineFunctions::Count => match data.deref() {
                                ArcTopicDataValue::Vec(vec) => {
                                    return if let Some(value) = BigDecimal::from_usize(vec.len()) {
                                        Ok(Arc::new(ArcTopicDataValue::Num(Arc::new(value))))
                                    } else {
                                        StdErrCode::DecimalParse.msg(format!(
                                            "Cannot retrieve[key={}, current={}] as decimal from [{:?}].",
                                            name, current_name, data
                                        ))
                                    };
                                }
                                ArcTopicDataValue::Map(map) => {
                                    return if let Some(value) = BigDecimal::from_usize(map.len()) {
                                        Ok(Arc::new(ArcTopicDataValue::Num(Arc::new(value))))
                                    } else {
                                        StdErrCode::DecimalParse.msg(format!(
                                            "Cannot retrieve[key={}, current={}] as decimal from [{:?}].",
                                            name, current_name, data
                                        ))
                                    };
                                }
                                _ => {
                                    return StdErrCode::VariableFuncNotSupported.msg(format!(
                                        "Cannot retrieve[key={}, current={}] from [{:?}].",
                                        name, current_name, data
                                    ));
                                }
                            },
                            VariablePredefineFunctions::Length => {}
                            VariablePredefineFunctions::Join => {}
                            VariablePredefineFunctions::Min => {}
                            VariablePredefineFunctions::Max => {}
                            VariablePredefineFunctions::Sum => {}
                            _ => {}
                        }
                    } else {
                    }
                }

                Ok(ARC__TOPIC_VALUE__NONE.clone())
            }
        }
    }
}
