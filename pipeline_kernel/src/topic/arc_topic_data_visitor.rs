use crate::{ArcTopicData, ArcTopicDataValue, ArcTopicDataVecValueMinmax, PipelineKernelErrorCode};
use bigdecimal::{BigDecimal, FromPrimitive};
use std::collections::HashMap;
use std::fmt::Debug;
use std::ops::Deref;
use std::sync::Arc;
use watchmen_model::{
    StdErr, StdErrCode, StdErrorCode, StdR, StringConverter, VariablePredefineFunctions,
};

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

impl ArcTopicDataValue {
    /// try to count, can only apply to vec or map
    /// otherwise raise error by given functions
    pub fn count<DecimalParseErr, NotSupport>(
        &self,
        decimal_parse_err: DecimalParseErr,
        not_support: NotSupport,
    ) -> StdR<Arc<ArcTopicDataValue>>
    where
        // decimal parse error
        DecimalParseErr: FnOnce() -> StdR<Arc<ArcTopicDataValue>>,
        // functions not supported
        NotSupport: FnOnce() -> StdR<Arc<ArcTopicDataValue>>,
    {
        match self {
            ArcTopicDataValue::Vec(vec) => BigDecimal::from_usize(vec.len())
                .map(|value| Ok(Arc::new(ArcTopicDataValue::Num(Arc::new(value)))))
                .unwrap_or(decimal_parse_err()),
            ArcTopicDataValue::Map(map) => BigDecimal::from_usize(map.len())
                .map(|value| Ok(Arc::new(ArcTopicDataValue::Num(Arc::new(value)))))
                .unwrap_or(decimal_parse_err()),
            _ => not_support(),
        }
    }

    /// get chars count of string, or decimal to string
    pub fn length<DecimalParseErr, NotSupport>(
        &self,
        decimal_parse_err: DecimalParseErr,
        not_support: NotSupport,
    ) -> StdR<Arc<ArcTopicDataValue>>
    where
        // decimal parse error
        DecimalParseErr: FnOnce() -> StdR<Arc<ArcTopicDataValue>>,
        // functions not supported
        NotSupport: FnOnce() -> StdR<Arc<ArcTopicDataValue>>,
    {
        match self {
            ArcTopicDataValue::Str(str) => BigDecimal::from_usize(str.chars().count())
                .map(|value| Ok(Arc::new(ArcTopicDataValue::Num(Arc::new(value)))))
                .unwrap_or(decimal_parse_err()),
            ArcTopicDataValue::Num(num) => {
                BigDecimal::from_usize(String::from_decimal(num).chars().count())
                    .map(|value| Ok(Arc::new(ArcTopicDataValue::Num(Arc::new(value)))))
                    .unwrap_or(decimal_parse_err())
            }
            _ => not_support(),
        }
    }

    /// distinct elements, can be applied on vec only
    /// for each element in vec,
    /// - str, num, datetime, date, time -> with the same type and value will be distinct,
    /// - bool -> maximum 2: true and false,
    /// - none -> maximum 1
    /// - vec, map -> cannot be removed as duplicates and are always added to the result.
    pub fn distinct<NotSupport>(&self, not_support: NotSupport) -> StdR<Arc<ArcTopicDataValue>>
    where
        // functions not supported
        NotSupport: FnOnce() -> StdR<Arc<ArcTopicDataValue>>,
    {
        match self {
            ArcTopicDataValue::Vec(vec) => {
                let mut distinct_values: Vec<Arc<ArcTopicDataValue>> = vec![];

                let mut none_added = false;
                let mut true_added = false;
                let mut false_added = false;
                let mut string_values = HashMap::new();
                let mut decimal_values = HashMap::new();
                let mut datetime_values = HashMap::new();
                let mut date_values = HashMap::new();
                let mut time_values = HashMap::new();

                vec.iter().for_each(|value| {
                    let should_add = match value.deref() {
                        ArcTopicDataValue::Str(str) => {
                            if !string_values.contains_key(str) {
                                string_values.insert(str.clone(), 1);
                                true
                            } else {
                                false
                            }
                        }
                        ArcTopicDataValue::Num(num) => {
                            if !decimal_values.contains_key(num) {
                                decimal_values.insert(num.clone(), 1);
                                true
                            } else {
                                false
                            }
                        }
                        ArcTopicDataValue::Bool(bool) => {
                            if *bool && !true_added {
                                true_added = true;
                                true
                            } else if !*bool && !false_added {
                                false_added = true;
                                true
                            } else {
                                false
                            }
                        }
                        ArcTopicDataValue::DateTime(datetime) => {
                            if !datetime_values.contains_key(datetime) {
                                datetime_values.insert(datetime.clone(), 1);
                                true
                            } else {
                                false
                            }
                        }
                        ArcTopicDataValue::Date(date) => {
                            if !date_values.contains_key(date) {
                                date_values.insert(date.clone(), 1);
                                true
                            } else {
                                false
                            }
                        }
                        ArcTopicDataValue::Time(time) => {
                            if !time_values.contains_key(time) {
                                time_values.insert(time.clone(), 1);
                                true
                            } else {
                                false
                            }
                        }
                        ArcTopicDataValue::Vec(_) => true,
                        ArcTopicDataValue::Map(_) => true,
                        ArcTopicDataValue::None => {
                            if !none_added {
                                none_added = true;
                                true
                            } else {
                                false
                            }
                        }
                    };
                    if should_add {
                        distinct_values.push(value.clone());
                    }
                });

                Ok(Arc::new(ArcTopicDataValue::Vec(Arc::new(distinct_values))))
            }
            _ => not_support(),
        }
    }

    /// 1. return cloned string when self is string
    /// 2. return joined string when self is vec, and element of vec cannot be vec or map. note the none value is ignored
    pub fn join<NotSupport>(
        &self,
        sep: &str,
        not_support: NotSupport,
    ) -> StdR<Arc<ArcTopicDataValue>>
    where
        // functions not supported
        NotSupport: FnOnce() -> StdR<Arc<ArcTopicDataValue>>,
    {
        match self {
            ArcTopicDataValue::Str(str) => Ok(Arc::new(ArcTopicDataValue::Str(str.clone()))),
            ArcTopicDataValue::Vec(vec) => {
                if vec.len() == 0 {
                    Ok(Arc::new(ArcTopicDataValue::Str(Arc::new("".to_string()))))
                } else {
                    let mut segments: Vec<String> = vec![];
                    for value in vec.iter() {
                        match value.deref() {
                            ArcTopicDataValue::Str(str) => {
                                segments.push(str.to_string());
                            }
                            ArcTopicDataValue::Num(num) => {
                                segments.push(String::from_decimal(num.deref()));
                            }
                            ArcTopicDataValue::Bool(bool) => {
                                segments.push(String::from_bool(bool));
                            }
                            ArcTopicDataValue::DateTime(datetime) => {
                                segments.push(String::from_datetime(datetime));
                            }
                            ArcTopicDataValue::Date(date) => {
                                segments.push(String::from_date(date));
                            }
                            ArcTopicDataValue::Time(time) => {
                                segments.push(String::from_time(time));
                            }
                            ArcTopicDataValue::None => {}
                            _ => return not_support(),
                        }
                    }
                    Ok(Arc::new(ArcTopicDataValue::Str(Arc::new(
                        segments.join(sep),
                    ))))
                }
            }
            _ => not_support(),
        }
    }

    /// get the min value of vec elements, only decimal/datetime/date/time/none can be compared
    /// - if there is no element in vec, returns none.
    /// - all elements must, can be converted to one single type,
    /// - if there are datetime and date, returns date.
    pub fn min<NotSupport>(&self, not_support: NotSupport) -> StdR<Arc<ArcTopicDataValue>>
    where
        // functions not supported
        NotSupport: Fn() -> StdErr,
    {
        match self {
            ArcTopicDataValue::Vec(vec) => vec.min_value(not_support),
            _ => Err(not_support()),
        }
    }

    pub fn max<NotSupport>(&self, not_support: NotSupport) -> StdR<Arc<ArcTopicDataValue>>
    where
        // functions not supported
        NotSupport: Fn() -> StdErr,
    {
        match self {
            ArcTopicDataValue::Vec(vec) => vec.max_value(not_support),
            _ => Err(not_support()),
        }
    }

    pub fn sum<NotSupport>(&self, _not_support: NotSupport) -> StdR<Arc<ArcTopicDataValue>>
    where
        // functions not supported
        NotSupport: FnOnce() -> StdR<Arc<ArcTopicDataValue>>,
    {
        todo!("implement sum for ArcTopicDataValue")
    }

    pub fn avg<NotSupport>(&self, _not_support: NotSupport) -> StdR<Arc<ArcTopicDataValue>>
    where
        // functions not supported
        NotSupport: FnOnce() -> StdR<Arc<ArcTopicDataValue>>,
    {
        todo!("implement avg for ArcTopicDataValue")
    }
}

pub trait TopicDataUtils {
    fn value_of(&self, property: &TopicDataProperty) -> StdR<Arc<ArcTopicDataValue>>;

    fn decimal_parse_error<R>(&self, name: &String, current_name: &String) -> StdR<R>
    where
        Self: Debug,
    {
        StdErrCode::DecimalParse.msg(format!(
            "Cannot retrieve[key={}, current={}] as decimal from [{:?}].",
            name, current_name, &self
        ))
    }

    fn function_not_supported<R>(&self, name: &String, current_name: &String) -> StdR<R>
    where
        Self: Debug,
    {
        Err(self.err_function_not_supported(name, current_name))
    }

    fn err_function_not_supported(&self, name: &String, current_name: &String) -> StdErr
    where
        Self: Debug,
    {
        PipelineKernelErrorCode::VariableFuncNotSupported.e_msg(format!(
            "Cannot retrieve[key={}, current={}] as decimal from [{:?}].",
            name, current_name, &self
        ))
    }
}

impl TopicDataUtils for ArcTopicData {
    fn value_of(&self, property: &TopicDataProperty) -> StdR<Arc<ArcTopicDataValue>> {
        match property {
            TopicDataProperty::Str((name, _)) => {
                // use none if name not exists, never mind the array or not
                let value = self.get(name).clone();
                if value.is_some() {
                    Ok(value.unwrap().clone())
                } else {
                    Ok(Arc::new(ArcTopicDataValue::None))
                }
            }
            TopicDataProperty::Vec((name, names, array)) => {
                let data = self.get(&names[0]);
                if data.is_none() {
                    return if *array {
                        Ok(Arc::new(ArcTopicDataValue::Vec(vec![].into())))
                    } else {
                        Ok(Arc::new(ArcTopicDataValue::None))
                    };
                }

                let mut data = data.unwrap();
                let mut remain_count = names.len() - 1;
                let mut current_index = 1;
                while current_index <= remain_count {
                    let current_name = &names[current_index];
                    if let Some(func) = VariablePredefineFunctions::try_parse(current_name) {
                        match func {
                            VariablePredefineFunctions::Count => {
                                return data.count(
                                    || self.decimal_parse_error(name, current_name),
                                    || self.function_not_supported(name, current_name),
                                );
                            }
                            VariablePredefineFunctions::Length
                            | VariablePredefineFunctions::Len => {
                                return data.length(
                                    || self.decimal_parse_error(name, current_name),
                                    || self.function_not_supported(name, current_name),
                                );
                            }
                            VariablePredefineFunctions::Join => {
                                return data
                                    .join(",", || self.function_not_supported(name, current_name));
                            }
                            VariablePredefineFunctions::Distinct => {
                                return data
                                    .distinct(|| self.function_not_supported(name, current_name));
                            }
                            VariablePredefineFunctions::Min => {
                                return data
                                    .min(|| self.err_function_not_supported(name, current_name));
                            }
                            VariablePredefineFunctions::Max => {
                                return data
                                    .max(|| self.err_function_not_supported(name, current_name));
                            }
                            VariablePredefineFunctions::Sum => {
                                return data
                                    .sum(|| self.function_not_supported(name, current_name));
                            }
                            VariablePredefineFunctions::Avg => {
                                return data
                                    .avg(|| self.function_not_supported(name, current_name));
                            }
                            _ => {}
                        }
                    } else {
                    }
                }

                Ok(Arc::new(ArcTopicDataValue::None))
            }
        }
    }
}
