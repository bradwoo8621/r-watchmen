use config::{FileStoredFormat, Format, Map, Value, ValueKind};
use ini::Ini;

#[derive(Debug, Clone)]
pub struct EnvFile;

impl Format for EnvFile {
    /// copy from [crate::config, /file/format/ini]
    fn parse(
        &self,
        uri: Option<&String>,
        text: &str,
    ) -> Result<Map<String, Value>, Box<dyn std::error::Error + Send + Sync>> {
        let mut map: Map<String, Value> = Map::new();
        let i = Ini::load_from_str(text)?;
        for (sec, prop) in i.iter() {
            match sec {
                Some(sec) => {
                    let mut sec_map: Map<String, Value> = Map::new();
                    for (k, v) in prop.iter() {
                        sec_map.insert(
                            k.to_owned(),
                            Value::new(uri, ValueKind::String(v.to_owned())),
                        );
                    }
                    map.insert(sec.to_owned(), Value::new(uri, ValueKind::Table(sec_map)));
                }
                None => {
                    for (k, v) in prop.iter() {
                        map.insert(
                            k.to_owned(),
                            Value::new(uri, ValueKind::String(v.to_owned())),
                        );
                    }
                }
            }
        }

        Ok(map)
    }
}

// A slice of extensions associated to this format, when an extension
// is omitted from a file source, these will be tried implicitly:
impl FileStoredFormat for EnvFile {
    fn file_extensions(&self) -> &'static [&'static str] {
        &["env"]
    }
}
