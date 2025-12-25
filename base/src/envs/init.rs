use crate::{EnvFile, ErrorCode, StdErrCode, StdR, VoidR};
use config::{Case, Config, Environment, File, FileFormat};
use std::path::Path;
use std::sync::RwLock;

static GLOBAL_CONFIG: RwLock<Option<Config>> = RwLock::new(None);

pub struct Envs;

impl Envs {
    fn set(config: Config) -> VoidR {
        let mut guard = GLOBAL_CONFIG
            .write()
            .or_else(|e| StdErrCode::ConfigGlobalInstanceLock.msg(e.to_string()))?;
        *guard = Some(config);

        Ok(())
    }

    pub fn init() -> VoidR {
        let config = Config::builder()
            .add_source(File::new("./.env", EnvFile).required(false))
            .add_source(Environment::with_convert_case(Case::UpperSnake))
            .build()
            .or_else(|e| StdErrCode::ConfigInit.msg(e.to_string()))?;

        Self::set(config)
    }

    pub fn with_files(files: Vec<String>) -> VoidR {
        if files.len() == 0 {
            return Self::init();
        }

        let mut builder = Config::builder();
        if files.len() == 0 {
            builder = builder.add_source(File::new("./.env", EnvFile).required(false));
        } else {
            for file in files {
                let path = Path::new(&file);
                let ext = if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
                    Some(ext)
                } else if let Some(name) = path.file_name().and_then(|s| s.to_str()) {
                    if name.starts_with(".") {
                        Some(name[1..name.len()].as_ref())
                    } else {
                        None
                    }
                } else {
                    None
                };
                builder = match ext {
                    Some("env") => builder.add_source(File::new(file.as_str(), EnvFile)),
                    Some("toml") => builder.add_source(File::new(file.as_str(), FileFormat::Toml)),
                    Some("json") => builder.add_source(File::new(file.as_str(), FileFormat::Json)),
                    Some("json5") => {
                        builder.add_source(File::new(file.as_str(), FileFormat::Json5))
                    }
                    Some("yaml") | Some("yml") => {
                        builder.add_source(File::new(file.as_str(), FileFormat::Yaml))
                    }
                    Some("ini") => builder.add_source(File::new(file.as_str(), FileFormat::Ini)),
                    Some("ron") => builder.add_source(File::new(file.as_str(), FileFormat::Ron)),
                    _ => {
                        return StdErrCode::ConfigFileFormatNotSupported
                            .msg(format!("Env file[{}] not supported yet.", file));
                    }
                };
            }
        }

        let config = builder
            .add_source(Environment::with_convert_case(Case::UpperSnake))
            .build()
            .or_else(|e| StdErrCode::ConfigInit.msg(e.to_string()))?;

        Envs::set(config)
    }
}

impl Envs {
    pub fn get_str(key: &str) -> StdR<Option<String>> {
        let guard = GLOBAL_CONFIG
            .read()
            .or_else(|e| StdErrCode::ConfigGlobalInstanceLock.msg(e.to_string()))?;
        if let Some(config) = guard.as_ref() {
            if let Ok(s) = config.get_string(key) {
                Ok(Some(s.clone()))
            } else {
                Ok(None)
            }
        } else {
            StdErrCode::ConfigInit.msg("Environment not initialized yet.")
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Envs;
    use std::env::{remove_var, set_var};

    #[test]
    fn test_0() {
        unsafe {
            set_var("TEST_KEY", "test value");
        }

        Envs::with_files(vec!["test/.env".to_string()]).expect("Failed to init environment");
        assert_eq!(Envs::get_str("TEST_KEY").unwrap().unwrap(), "test value");

        unsafe {
            remove_var("TEST_KEY");
        }
    }
}
