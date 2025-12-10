use crate::ArcTopicDataValue;
use bigdecimal::BigDecimal;
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use std::ops::Deref;
use std::sync::Arc;
use watchmen_model::{DateTimeUtils, NumericUtils, StdErr, StdR, StringUtils};

trait MinmaxComparator<V: PartialOrd> {
    fn replace_self_if_less_than(&mut self, another: &Arc<V>);
    fn replace_self_if_greater_than(&mut self, another: &Arc<V>);

    fn replace_self_if(&mut self, another: &Arc<V>, compare: &MinmaxCompare) {
        match compare {
            MinmaxCompare::Less => self.replace_self_if_greater_than(another),
            MinmaxCompare::Greater => self.replace_self_if_less_than(another),
        }
    }
}

impl<V: PartialOrd> MinmaxComparator<V> for Option<Arc<V>> {
    /// returns min value between self and another
    fn replace_self_if_less_than(&mut self, another: &Arc<V>) {
        if let Some(one) = self {
            if one.deref() > another {
            } else {
                *self = Some(another.clone())
            }
        } else {
            *self = Some(another.clone())
        }
    }

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
enum MinmaxCompare {
    Less,
    Greater,
}

#[derive(Debug)]
struct Minmax {
    compare: MinmaxCompare,

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
    fn new(compare: MinmaxCompare) -> Self {
        Minmax {
            compare,

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

    fn with_decimal<NotSupport>(
        &mut self,
        decimal: &Arc<BigDecimal>,
        not_support: &NotSupport,
    ) -> StdR<bool>
    where
        NotSupport: Fn() -> StdErr,
    {
        if self.has_datetime || self.has_date || self.has_time {
            Err(not_support())
        } else {
            self.has_decimal = true;
            self.min_decimal.replace_self_if(decimal, &self.compare);
            Ok(true)
        }
    }

    fn with_datetime<NotSupport>(
        &mut self,
        datetime: &Arc<NaiveDateTime>,
        not_support: &NotSupport,
    ) -> StdR<bool>
    where
        NotSupport: Fn() -> StdErr,
    {
        if self.has_decimal || self.has_time {
            Err(not_support())
        } else {
            self.has_datetime = true;
            self.min_datetime.replace_self_if(datetime, &self.compare);
            // datetime also can be compared with date
            self.min_date
                .replace_self_if(&Arc::new(datetime.date()), &self.compare);
            Ok(true)
        }
    }

    fn with_date<NotSupport>(
        &mut self,
        date: &Arc<NaiveDate>,
        not_support: &NotSupport,
    ) -> StdR<bool>
    where
        NotSupport: Fn() -> StdErr,
    {
        if self.has_decimal || self.has_time {
            Err(not_support())
        } else {
            self.has_date = true;
            self.min_date.replace_self_if(date, &self.compare);
            Ok(true)
        }
    }

    fn with_time<NotSupport>(
        &mut self,
        time: &Arc<NaiveTime>,
        not_support: &NotSupport,
    ) -> StdR<bool>
    where
        NotSupport: Fn() -> StdErr,
    {
        if self.has_decimal || self.has_datetime || self.has_date {
            Err(not_support())
        } else {
            self.has_time = true;
            self.min_time.replace_self_if(time, &self.compare);
            Ok(true)
        }
    }

    /// 1. process when [has_decimal] is true
    ///    - error when not support detected
    ///    - true when handled
    /// 2. false when [has_decimal] is false
    fn with_decimal_by_string_elements<NotSupport>(
        &mut self,
        not_support: &NotSupport,
    ) -> StdR<bool>
    where
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

    /// 1. process when [has_date] is true
    ///    - error when not support detected
    ///    - true when handled
    /// 2. false when [has_date] is false
    fn with_date_by_string_elements<NotSupport>(&mut self, not_support: &NotSupport) -> StdR<bool>
    where
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

    /// 1. process when [has_datetime] is true
    ///    - error when not support detected
    ///    - true when handled
    /// 2. false when [has_datetime] is false
    fn with_datetime_by_string_elements<NotSupport>(
        &mut self,
        not_support: &NotSupport,
    ) -> StdR<bool>
    where
        NotSupport: Fn() -> StdErr,
    {
        let mut downgrade_to_date = false;
        if self.has_datetime {
            for value in self.string_elements.clone().iter() {
                if !downgrade_to_date && let Ok(datetime) = value.to_datetime() {
                    self.with_datetime(&Arc::new(datetime), not_support)?;
                } else if let Ok(date) = value.to_date() {
                    downgrade_to_date = true;
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

    /// 1. process when [has_time] is true
    ///    - error when not support detected
    ///    - true when handled
    /// 2. false when [has_time] is false
    fn with_time_by_string_elements<NotSupport>(&mut self, not_support: &NotSupport) -> StdR<bool>
    where
        NotSupport: Fn() -> StdErr,
    {
        if self.has_time {
            for value in self.string_elements.clone().iter() {
                if let Ok(time) = value.to_time() {
                    self.with_time(&Arc::new(time), not_support)?;
                } else {
                    return Err(not_support());
                }
            }
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// 1. process when none of the known types including decimal, datetime, date, time, or none is detected.
    ///    - error when not support detected
    ///    - true when handled
    /// 2. false when one of the known types detected.
    ///
    /// If there are no separators in time data (datetime/date/time), it may be recognized as a decimal.
    /// Therefore, in the process of automatically recognizing strings,
    /// it's impossible to avoid the possibility of misrecognition.
    /// In this implementation, decimal has the highest priority for recognition,
    /// followed by datetime, date, and time in sequence.
    /// Meanwhile, if a date is recognized, the previously recognized datetime will be automatically downgraded to a date.
    fn with_unknown_by_string_elements<NotSupport>(
        &mut self,
        not_support: &NotSupport,
    ) -> StdR<bool>
    where
        NotSupport: Fn() -> StdErr,
    {
        if self.has_decimal || self.has_datetime || self.has_date || self.has_time {
            return Ok(false);
        }

        for value in self.string_elements.clone().iter() {
            if !(self.has_datetime || self.has_date || self.has_time) {
                if let Ok(decimal) = value.to_decimal() {
                    self.with_decimal(&Arc::new(decimal), not_support)?;
                    continue;
                }
            }
            if !(self.has_decimal || self.has_time) {
                if !self.has_date
                    && let Ok(datetime) = value.to_datetime()
                {
                    self.with_datetime(&Arc::new(datetime), not_support)?;
                    continue;
                } else if let Ok(date) = value.to_date_loose() {
                    self.with_date(&Arc::new(date), not_support)?;
                    continue;
                }
            }
            if !(self.has_decimal || self.has_datetime || self.has_date) {
                if let Ok(time) = value.to_time() {
                    self.with_time(&Arc::new(time), not_support)?;
                    continue;
                }
            }
        }
        Ok(true)
    }

    /// false means not supported detected
    fn handle_string_elements<NotSupport>(&mut self, not_support: &NotSupport) -> StdR<bool>
    where
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
        Ok(true)
    }

    fn get_result<NotSupport>(&self, not_support: &NotSupport) -> StdR<Arc<ArcTopicDataValue>>
    where
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

/// - Supports multiple formats: decimal/datetime/date/time.
/// - No element is considered returning `none`,
/// - When calculating the minimum value, `none` and empty string are considered the minimum value `none`.
/// - When calculating the maximum value, `none` and empty string are always ignored.
/// - Blank string always leads an error,
/// - Only the extreme value of a single type will be calculated. If the data contains multiple types, an error will be reported.
///   Unless it only contains both `datetime` and `date` simultaneously.
///   In this case, the `datetime` data will be downgraded to `date`, and the calculated extreme value will also be of the `date` type.
/// - String values will be automatically converted to the detected type. If the conversion fails, an error will be reported.
/// - If there are no explicit values of the four types and `none`,
///   but there are string values, the conversion will be carried out in the priority order of
///   decimal > datetime > date > time, and the compatibility rules are the same as above.
///   Note that once `date` is detected, the already detected `datetime` will also be downgraded.
trait ArcTopicDataVecValueMinmaxFinder {
    fn find<NotSupport>(
        self,
        minmax: Minmax,
        not_support: NotSupport,
    ) -> StdR<Arc<ArcTopicDataValue>>
    where
        NotSupport: Fn() -> StdErr;
}

impl ArcTopicDataVecValueMinmaxFinder for &Arc<Vec<Arc<ArcTopicDataValue>>> {
    fn find<NotSupport>(
        self,
        mut minmax: Minmax,
        not_support: NotSupport,
    ) -> StdR<Arc<ArcTopicDataValue>>
    where
        NotSupport: Fn() -> StdErr,
    {
        if self.len() == 0 {
            return Ok(Arc::new(ArcTopicDataValue::None));
        }

        for value in self.iter() {
            match value.deref() {
                ArcTopicDataValue::None => {
                    // none is the min value
                    if let MinmaxCompare::Less = minmax.compare {
                        return Ok(Arc::new(ArcTopicDataValue::None));
                    }
                }
                ArcTopicDataValue::Str(str) => {
                    if str.is_empty() {
                        // empty string treated as none, and none is the min value
                        if let MinmaxCompare::Less = minmax.compare {
                            return Ok(Arc::new(ArcTopicDataValue::None));
                        }
                    } else if str.is_blank() {
                        // obviously that blank string cannot be cast to any comparable type
                        // error
                        return Err(not_support());
                    } else {
                        // postpone, detect types first
                        minmax.string_elements.push(str.clone())
                    }
                }
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
        minmax.get_result(&not_support)
    }
}

pub trait ArcTopicDataVecValueMinmax {
    fn max_value<NotSupport>(self, not_support: NotSupport) -> StdR<Arc<ArcTopicDataValue>>
    where
        NotSupport: Fn() -> StdErr;

    fn min_value<NotSupport>(self, not_support: NotSupport) -> StdR<Arc<ArcTopicDataValue>>
    where
        NotSupport: Fn() -> StdErr;
}

impl ArcTopicDataVecValueMinmax for &Arc<Vec<Arc<ArcTopicDataValue>>> {
    fn max_value<NotSupport>(self, not_support: NotSupport) -> StdR<Arc<ArcTopicDataValue>>
    where
        NotSupport: Fn() -> StdErr,
    {
        self.find(Minmax::new(MinmaxCompare::Greater), not_support)
    }

    fn min_value<NotSupport>(self, not_support: NotSupport) -> StdR<Arc<ArcTopicDataValue>>
    where
        NotSupport: Fn() -> StdErr,
    {
        self.find(Minmax::new(MinmaxCompare::Less), not_support)
    }
}

#[cfg(test)]
mod tests {
    use crate::topic::arc_topic_data_minmax::{Minmax, MinmaxCompare};
    use crate::{ArcTopicDataValue, ArcTopicDataVecValueMinmax, PipelineKernelErrorCode};
    use bigdecimal::BigDecimal;
    use chrono::{NaiveDate, NaiveDateTime};
    use std::ops::Deref;
    use std::str::FromStr;
    use std::sync::Arc;
    use watchmen_model::StdErrorCode;

    #[test]
    fn test() {
        let mut minmax = Minmax::new(MinmaxCompare::Less);
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

    #[test]
    fn test_all_decimal() {
        let vec: Vec<Arc<ArcTopicDataValue>> = vec![
            ArcTopicDataValue::Num(Arc::new(BigDecimal::from_str("100").unwrap())),
            ArcTopicDataValue::Num(Arc::new(BigDecimal::from_str("200").unwrap())),
            ArcTopicDataValue::Str(Arc::new("50".to_string())),
        ]
        .into_iter()
        .map(Arc::new)
        .collect();
        let vec = &Arc::new(vec);
        let min = vec.min_value(|| PipelineKernelErrorCode::VariableFuncNotSupported.e());
        assert!(min.is_ok());
        let min = min.unwrap();
        assert!(matches!(min.deref(), &ArcTopicDataValue::Num(_)));
        match min.deref() {
            ArcTopicDataValue::Num(v) => {
                assert_eq!(v.to_plain_string(), "50")
            }
            _ => panic!("wrong"),
        }

        println!("{:?}", vec[2]);
    }

    #[test]
    fn test_date_and_datetime() {
        let vec: Vec<Arc<ArcTopicDataValue>> = vec![
            ArcTopicDataValue::Date(Arc::new(NaiveDate::from_ymd_opt(2024, 1, 10).unwrap())),
            ArcTopicDataValue::DateTime(Arc::new(
                NaiveDateTime::parse_from_str("2024-01-02 01:02:03", "%Y-%m-%d %H:%M:%S").unwrap(),
            )),
        ]
        .into_iter()
        .map(Arc::new)
        .collect();
        let vec = &Arc::new(vec);
        let min = vec.min_value(|| PipelineKernelErrorCode::VariableFuncNotSupported.e());
        assert!(min.is_ok());
        let min = min.unwrap();
        assert!(matches!(min.deref(), &ArcTopicDataValue::Date(_)));
        match min.deref() {
            ArcTopicDataValue::Date(v) => {
                assert_eq!(v.format("%Y-%m-%d").to_string().as_str(), "2024-01-02")
            }
            _ => panic!("wrong"),
        }

        println!("{:?}", vec);
    }
}
