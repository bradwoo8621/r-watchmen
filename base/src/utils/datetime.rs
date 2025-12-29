use crate::{
    DateFormats, DateTimeFormat, DateTimeFormats, ErrorCode, Formats, StdErrCode, StdR, TimeFormats,
};
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use std::sync::Arc;

struct DateTimeInnerUtils;

impl DateTimeInnerUtils {
    /// get digit chars of given string
    /// returns collected digit chars and count
    fn digits_of(str: &String) -> (String, usize) {
        let mut s = String::new();
        for char in str.chars() {
            if char.is_ascii_digit() {
                s.push(char);
            }
        }
        let count = s.chars().count();
        (s, count)
    }

    fn parse<T, GetFormats, TryParse, ParseFail>(
        str: &String,
        get_formats: GetFormats,
        try_parse: TryParse,
        parse_fail: ParseFail,
    ) -> StdR<T>
    where
        GetFormats: FnOnce(usize) -> Vec<Arc<DateTimeFormat>>,
        TryParse: Fn(&str, &DateTimeFormat) -> Option<T>,
        ParseFail: FnOnce(String) -> StdR<T>,
    {
        let (s, count) = DateTimeInnerUtils::digits_of(str);
        let formats = get_formats(count);
        for format in formats.iter() {
            if let Some(t) = try_parse(s.as_str(), &format) {
                return Ok(t);
            }
        }

        parse_fail(s)
    }
}

pub trait DateTimeUtils {
    fn to_date(&self) -> StdR<NaiveDate>;
    /// to date, when string can be [to_date] or [to_datetime],
    /// the time part will be discarded
    fn to_date_loose(&self) -> StdR<NaiveDate>;
    fn to_time(&self) -> StdR<NaiveTime>;
    fn to_datetime(&self) -> StdR<NaiveDateTime>;
}

impl DateTimeUtils for String {
    fn to_date(&self) -> StdR<NaiveDate> {
        DateTimeInnerUtils::parse(
            &self,
            |count| DateFormats::formats_of(count),
            |s, format| {
                if let Ok(date) = NaiveDate::parse_from_str(s, &format.format) {
                    Some(date)
                } else {
                    None
                }
            },
            |s| StdErrCode::TimeParse.msg(format!("Cannot parse [{}] to date.", s)),
        )
    }

    fn to_date_loose(&self) -> StdR<NaiveDate> {
        let (s, count) = DateTimeInnerUtils::digits_of(&self);
        let formats = DateFormats::formats_of(count);
        for format in formats.iter() {
            if let Ok(t) = NaiveDate::parse_from_str(s.as_str(), &format.format) {
                return Ok(t);
            }
        }

        let formats = DateTimeFormats::formats_of(count);
        for format in formats.iter() {
            if let Ok(t) = NaiveDateTime::parse_from_str(s.as_str(), &format.format) {
                return Ok(t.date());
            }
        }

        StdErrCode::TimeParse.msg(format!("Cannot parse [{}] to date.", s))
    }

    fn to_time(&self) -> StdR<NaiveTime> {
        DateTimeInnerUtils::parse(
            &self,
            |count| TimeFormats::formats_of(count),
            |s, format| {
                if let Ok(time) = NaiveTime::parse_from_str(s, &format.format) {
                    Some(time)
                } else {
                    None
                }
            },
            |s| StdErrCode::TimeParse.msg(format!("Cannot parse [{}] to time.", s)),
        )
    }

    fn to_datetime(&self) -> StdR<NaiveDateTime> {
        DateTimeInnerUtils::parse(
            &self,
            |count| DateTimeFormats::formats_of(count),
            |s, format| {
                if let Ok(datetime) = NaiveDateTime::parse_from_str(s, &format.format) {
                    Some(datetime)
                } else {
                    None
                }
            },
            |s| StdErrCode::TimeParse.msg(format!("Cannot parse [{}] to datetime.", s)),
        )
    }
}
