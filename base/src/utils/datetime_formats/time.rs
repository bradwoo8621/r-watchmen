use crate::{DateTimeFormat, DateTimeFormatMap, Formats};
use std::sync::{Arc, OnceLock};

static DEFAULT_TIME_FORMATS: OnceLock<Arc<Vec<Arc<String>>>> = OnceLock::new();

pub struct TimeFormats;

impl Formats for TimeFormats {
    fn env_key() -> &'static str {
        "TIME_FORMATS"
    }

    fn default_formats() -> &'static Arc<Vec<Arc<String>>> {
        DEFAULT_TIME_FORMATS.get_or_init(|| {
            let formats = Arc::new(
                vec![
                    // 6 digits
                    "%H%M%S", // 4 digits
                    "%H%M",
                ]
                .iter()
                .map(|s| Arc::new(s.to_string()))
                .collect(),
            );
            DateTimeFormatMap::compute_formats(&formats);
            formats
        })
    }

    fn ok_with(format: &DateTimeFormat, len: usize) -> bool {
        format.len == len
    }
}
