use std::collections::HashMap;
use std::ops::Deref;
use std::sync::{Arc, OnceLock, RwLock};

pub struct DateTimeFormatMap;

pub struct DateTimeFormat {
    pub format: String,
    pub len: usize,
    /// has millisecond ([%f, %3f]) or not
    pub has_milli: bool,
    /// has timezone ([%z]) or not
    pub has_tz: bool,
}

type FormatMap = HashMap<String, Arc<DateTimeFormat>>;

static FORMAT_MAP: OnceLock<RwLock<FormatMap>> = OnceLock::new();

impl DateTimeFormatMap {
    fn map() -> &'static RwLock<FormatMap> {
        FORMAT_MAP.get_or_init(|| RwLock::new(HashMap::new()))
    }

    fn get_format_from_map(
        format: &String,
        lens: &RwLock<FormatMap>,
    ) -> Option<Arc<DateTimeFormat>> {
        if let Ok(guard) = lens.read() {
            if let Some(detail) = guard.get(format) {
                Some(detail.clone())
            } else {
                None
            }
        } else {
            None
        }
    }

    fn get_or_compute_format(format: &String, lens: &RwLock<FormatMap>) -> Arc<DateTimeFormat> {
        match Self::get_format_from_map(format, lens) {
            Some(detail) => detail,
            _ => {
                let mut len: usize = 0;
                let mut has_millisecond = false;
                let mut has_timezone = false;
                let mut parsed_format = String::new();
                for parts in format.split('%') {
                    match parts {
                        "Y" => {
                            len += 4;
                            parsed_format.push('%');
                            parsed_format.push('Y')
                        }
                        "m" | "d" | "H" | "M" | "S" => {
                            len += 2;
                            parsed_format.push('%');
                            parsed_format.push(parts.chars().nth(0).unwrap());
                        }
                        "f" | "3f" => {
                            len += 3;
                            has_millisecond = true;
                            parsed_format.push('%');
                            parsed_format.push('3');
                            parsed_format.push('f')
                        }
                        "z" => {
                            len += 5;
                            has_timezone = true;
                            parsed_format.push('%');
                            parsed_format.push('z')
                        }
                        // not supported, ignored, just make the length to 100
                        _ => len += 100,
                    }
                }

                let detail = Arc::new(DateTimeFormat {
                    format: parsed_format,
                    len,
                    has_milli: has_millisecond,
                    has_tz: has_timezone,
                });
                if let Ok(mut guard) = lens.write() {
                    guard.insert(format.clone(), detail.clone());
                }
                detail
            }
        }
    }

    pub fn compute_formats(formats: &Vec<Arc<String>>) {
        let lens = Self::map();
        for format in formats {
            Self::get_or_compute_format(format.deref(), lens);
        }
    }

    pub fn get_format(format: &String) -> Arc<DateTimeFormat> {
        Self::get_or_compute_format(format, Self::map())
    }
}
