use crate::{DateTimeFormat, DateTimeFormatMap, Envs};
use std::sync::Arc;

pub trait Formats {
    fn env_key() -> &'static str;

    fn default_formats() -> &'static Arc<Vec<Arc<String>>>;

    fn formats() -> Vec<Arc<DateTimeFormat>> {
        Envs::str_vec_or(Self::env_key(), Self::default_formats())
            .iter()
            .map(|f| DateTimeFormatMap::get_format(f))
            .collect()
    }

    fn ok_with(format: &DateTimeFormat, len: usize) -> bool {
        if len > 14 {
            // has datetime, and more
            if format.has_tz {
                // has timezone
                if len > 19 {
                    // and more, should be millisecond
                    format.has_milli
                } else {
                    format.len == len
                }
            } else {
                // must have millisecond
                format.has_milli
            }
        } else {
            format.len == len
        }
    }

    /// get formats of given length, returns empty vec when no suitable format
    fn formats_of(len: usize) -> Vec<Arc<DateTimeFormat>> {
        Self::formats()
            .iter()
            .filter(|f| Self::ok_with(f, len))
            .map(|f| f.clone())
            .collect()
    }
}
