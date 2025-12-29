use crate::{DateTimeFormatMap, Formats};
use std::sync::{Arc, OnceLock};

static DEFAULT_FULL_DATETIME_FORMATS: OnceLock<Arc<Vec<Arc<String>>>> = OnceLock::new();

pub struct FullDateTimeFormats;

impl Formats for FullDateTimeFormats {
    fn env_key() -> &'static str {
        "FULL_DATETIME_FORMATS"
    }

    fn default_formats() -> &'static Arc<Vec<Arc<String>>> {
        DEFAULT_FULL_DATETIME_FORMATS.get_or_init(|| {
            let formats = Arc::new(
                vec![
                    // 14 or more digits
                    "%Y%m%d%H%M%S%f",
                    "%d%m%Y%H%M%S%f",
                    "%m%d%Y%H%M%S%f",
                ]
                .iter()
                .map(|s| Arc::new(s.to_string()))
                .collect(),
            );
            DateTimeFormatMap::compute_formats(&formats);
            formats
        })
    }
}
