use crate::{DateTimeFormatMap, Formats};
use std::sync::{Arc, OnceLock};

static DEFAULT_DATE_FORMATS: OnceLock<Arc<Vec<Arc<String>>>> = OnceLock::new();
pub struct DateFormats;

impl Formats for DateFormats {
    fn env_key() -> &'static str {
        "DATE_FORMATS"
    }

    fn default_formats() -> &'static Arc<Vec<Arc<String>>> {
        DEFAULT_DATE_FORMATS.get_or_init(|| {
            let formats = Arc::new(
                vec![
                    // 8 digits
                    "%Y%m%d", "%d%m%Y", "%m%d%Y",
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
