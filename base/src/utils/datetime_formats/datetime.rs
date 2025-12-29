use crate::{DateTimeFormatMap, Formats};
use std::sync::{Arc, OnceLock};

static DEFAULT_DATETIME_FORMATS: OnceLock<Arc<Vec<Arc<String>>>> = OnceLock::new();

pub struct DateTimeFormats;

impl Formats for DateTimeFormats {
    fn env_key() -> &'static str {
        "DATETIME_FORMATS"
    }

    fn default_formats() -> &'static Arc<Vec<Arc<String>>> {
        DEFAULT_DATETIME_FORMATS.get_or_init(|| {
            let formats = Arc::new(
                vec![
                    // 14 digits
                    "%Y%m%d%H%M%S",
                    "%d%m%Y%H%M%S",
                    "%m%d%Y%H%M%S",
                    // 12 digits
                    "%Y%m%d%H%M",
                    "%d%m%Y%H%M",
                    "%m%d%Y%H%M",
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
