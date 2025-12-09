use crate::ArcTopicDataValue;
use bigdecimal::BigDecimal;
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use std::ops::Deref;
use std::sync::Arc;
use watchmen_model::{DateTimeUtils, NumericUtils, StdErr, StdR};

trait MinmaxComparator<V: PartialOrd> {
    fn replace_self_if_greater_than(&mut self, b: &Arc<V>);
}

impl<V: PartialOrd> MinmaxComparator<V> for Option<Arc<V>> {
    /// returns min value between self and another
    fn replace_self_if_greater_than(&mut self, another: &Arc<V>) {
        if let Some(one) = self {
            if one.deref() < another {
            } else {
                *self = Some(another.clone())
            }
        } else {
            *self = Some(another.clone())
        }
    }
}

#[derive(Debug)]
struct Minmax {
    has_decimal: bool,
    min_decimal: Option<Arc<BigDecimal>>,
    has_datetime: bool,
    min_datetime: Option<Arc<NaiveDateTime>>,
    has_date: bool,
    min_date: Option<Arc<NaiveDate>>,
    has_time: bool,
    min_time: Option<Arc<NaiveTime>>,

    string_elements: Vec<Arc<String>>,
}

impl Minmax {
    fn new() -> Self {
        Minmax {
            has_decimal: false,
            min_decimal: None,
            has_datetime: false,
            min_datetime: None,
            has_date: false,
            min_date: None,
            has_time: false,
            min_time: None,

            string_elements: vec![],
        }
    }

    /// false means not supported detected
    fn with_decimal<NotSupport>(
        &mut self,
        decimal: &Arc<BigDecimal>,
        not_support: &NotSupport,
    ) -> StdR<bool>
    where
        // functions not supported
        NotSupport: Fn() -> StdErr,
    {
        if self.has_datetime || self.has_date || self.has_time {
            Err(not_support())
        } else {
            self.has_decimal = true;
            self.min_decimal.replace_self_if_greater_than(decimal);
            Ok(true)
        }
    }

    /// false means not supported detected
    fn with_datetime<NotSupport>(
        &mut self,
        datetime: &Arc<NaiveDateTime>,
        not_support: &NotSupport,
    ) -> StdR<bool>
    where
        // functions not supported
        NotSupport: Fn() -> StdErr,
    {
        if self.has_decimal || self.has_time {
            Err(not_support())
        } else {
            self.has_datetime = true;
            self.min_datetime.replace_self_if_greater_than(datetime);
            // datetime also can be compared with date
            self.min_date
                .replace_self_if_greater_than(&Arc::new(datetime.date()));
            Ok(true)
        }
    }

    /// false means not supported detected
    fn with_date<NotSupport>(
        &mut self,
        date: &Arc<NaiveDate>,
        not_support: &NotSupport,
    ) -> StdR<bool>
    where
        // functions not supported
        NotSupport: Fn() -> StdErr,
    {
        if self.has_decimal || self.has_time {
            Err(not_support())
        } else {
            self.has_date = true;
            self.min_date.replace_self_if_greater_than(date);
            Ok(true)
        }
    }

    /// false means not supported detected
    fn with_time<NotSupport>(
        &mut self,
        time: &Arc<NaiveTime>,
        not_support: &NotSupport,
    ) -> StdR<bool>
    where
        // functions not supported
        NotSupport: Fn() -> StdErr,
    {
        if self.has_decimal || self.has_datetime || self.has_date {
            Err(not_support())
        } else {
            self.has_time = true;
            self.min_time.replace_self_if_greater_than(time);
            Ok(true)
        }
    }

    /// false means not supported detected
    /// 1. process when [has_decimal] is true
    ///    - error when not support detected
    ///    - true when handled
    /// 2. false when [has_decimal] is false
    fn with_decimal_by_string_elements<NotSupport>(
        &mut self,
        not_support: &NotSupport,
    ) -> StdR<bool>
    where
        // functions not supported
        NotSupport: Fn() -> StdErr,
    {
        if self.has_decimal {
            for value in self.string_elements.clone().iter() {
                if let Ok(decimal) = value.to_decimal() {
                    self.with_decimal(&Arc::new(decimal), not_support)?;
                } else {
                    return Err(not_support());
                }
            }
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// false means not supported detected
    /// 1. process when [has_date] is true
    ///    - error when not support detected
    ///    - true when handled
    /// 2. false when [has_date] is false
    fn with_date_by_string_elements<NotSupport>(&mut self, not_support: &NotSupport) -> StdR<bool>
    where
        // functions not supported
        NotSupport: Fn() -> StdErr,
    {
        if self.has_date {
            for value in self.string_elements.clone().iter() {
                if let Ok(date) = value.to_date_loose() {
                    self.with_date(&Arc::new(date), not_support)?;
                } else {
                    return Err(not_support());
                }
            }
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// false means not supported detected
    fn handle_string_elements<NotSupport>(&mut self, not_support: &NotSupport) -> StdR<bool>
    where
        // functions not supported
        NotSupport: Fn() -> StdErr,
    {
        if self.string_elements.len() == 0 {
            return Ok(true);
        }

        let mut handled = self.with_decimal_by_string_elements(not_support)?;
        if !handled {
            handled = self.with_date_by_string_elements(not_support)?;
        }
        if !handled {
            handled = self.with_datetime_by_string_elements(not_support)?;
        }
        if !handled {
            handled = self.with_time_by_string_elements(not_support)?;
        }
        if !handled {
            self.with_unknown_by_string_elements(not_support)?;
        }
        // } else if has_datetime {
        //     let mut downgrade_to_date = false;
        //     // There will still be strings that can only be parsed as the date type and cannot be parsed as datetime.
        //     // When this situation occurs, it will be downgraded to finding the minimum value of the dates.
        //     for value in string_values.iter() {
        //         if !downgrade_to_date && let Ok(datetime) = value.to_datetime() {
        //             min_datetime = datetime.min_of(min_datetime);
        //         } else if let Ok(date) = value.to_date() {
        //             if !downgrade_to_date {
        //                 downgrade_to_date = true;
        //                 min_date = date.min_of(min_datetime);
        //             } else {
        //                 min_date = date.min_of(min_date);
        //             }
        //         } else {
        //             return f1();
        //         }
        //     }
        // } else if has_time {
        //     for value in string_values.iter() {
        //         if let Ok(time) = value.to_time() {
        //             min_time = time.min_of(min_time);
        //         } else {
        //             return f1();
        //         }
        //     }
        // } else {
        //     // TODO no decimal/datetime/date/time, tricky thing!
        //     return not_support();
        // }
        Ok(true)
    }

    fn try_get_min_value<NotSupport>(
        &self,
        not_support: &NotSupport,
    ) -> StdR<Arc<ArcTopicDataValue>>
    where
        // functions not supported
        NotSupport: Fn() -> StdErr,
    {
        if self.has_decimal {
            Ok(Arc::new(ArcTopicDataValue::Num(
                self.min_decimal.as_ref().unwrap().clone(),
            )))
        } else if self.has_date {
            Ok(Arc::new(ArcTopicDataValue::Date(
                self.min_date.as_ref().unwrap().clone(),
            )))
        } else if self.has_datetime {
            Ok(Arc::new(ArcTopicDataValue::DateTime(
                self.min_datetime.as_ref().unwrap().clone(),
            )))
        } else if self.has_time {
            Ok(Arc::new(ArcTopicDataValue::Time(
                self.min_time.as_ref().unwrap().clone(),
            )))
        } else {
            // no decimal/datetime/date/time/string,
            // function is not supported, since don't know how to compare
            Err(not_support())
        }
    }
}

pub trait ArcTopicDataValueMinmax {
    fn min_of<NotSupport>(
        vec: &Arc<Vec<Arc<ArcTopicDataValue>>>,
        not_support: NotSupport,
    ) -> StdR<Arc<ArcTopicDataValue>>
    where
        // functions not supported
        NotSupport: Fn() -> StdErr,
    {
        if vec.len() == 0 {
            return Ok(Arc::new(ArcTopicDataValue::None));
        }

        let mut minmax = Minmax::new();

        for value in vec.iter() {
            match value.deref() {
                ArcTopicDataValue::None => {
                    return Ok(Arc::new(ArcTopicDataValue::None));
                }
                ArcTopicDataValue::Str(str) => minmax.string_elements.push(str.clone()),
                ArcTopicDataValue::Num(decimal) => {
                    minmax.with_decimal(decimal, &not_support)?;
                }
                ArcTopicDataValue::DateTime(datetime) => {
                    minmax.with_datetime(datetime, &not_support)?;
                }
                ArcTopicDataValue::Date(date) => {
                    minmax.with_date(date, &not_support)?;
                }
                ArcTopicDataValue::Time(time) => {
                    minmax.with_time(time, &not_support)?;
                }
                _ => return Err(not_support()),
            }
        }

        minmax.handle_string_elements(&not_support)?;
        minmax.try_get_min_value(&not_support)
    }
}

impl ArcTopicDataValueMinmax for ArcTopicDataValue {}

#[cfg(test)]
mod tests {
    use crate::topic::arc_topic_data_minmax::Minmax;
    use crate::PipelineKernelErrorCode;
    use bigdecimal::BigDecimal;
    use std::str::FromStr;
    use std::sync::Arc;
    use watchmen_model::StdErrorCode;

    #[test]
    fn test() {
        let mut minmax = Minmax::new();
        let not_support = || PipelineKernelErrorCode::VariableFuncNotSupported.e();
        minmax
            .with_decimal(
                &Arc::new(BigDecimal::from_str("100").unwrap()),
                &not_support,
            )
            .expect("100 not supported");
        minmax
            .with_decimal(
                &Arc::new(BigDecimal::from_str("200").unwrap()),
                &not_support,
            )
            .expect("200 not supported");
        println!("{:?}", minmax);
    }
}
