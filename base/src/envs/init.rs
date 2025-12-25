use crate::{EnvFile, ErrorCode, StdErrCode, StdR, VoidR};
use config::{Case, Config, Environment, File};
use std::sync::RwLock;

static GLOBAL_CONFIG: RwLock<Option<Config>> = RwLock::new(None);

pub struct Envs;

impl Envs {
    fn default_env() -> StdR<Config> {
        Config::builder()
            .add_source(File::new("./.env", EnvFile).required(false))
            .add_source(Environment::with_convert_case(Case::UpperSnake))
            .build()
            .or_else(|e| StdErrCode::ConfigInit.msg(e.to_string()))
    }

    pub fn init() -> VoidR {
        let config = Envs::default_env()?;

        let mut guard = GLOBAL_CONFIG
            .write()
            .or_else(|e| StdErrCode::ConfigGlobalInstanceLock.msg(e.to_string()))?;
        *guard = Some(config);

        Ok(())
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

        Envs::init().expect("Failed to init environment");
        assert_eq!(Envs::get_str("TEST_KEY").unwrap().unwrap(), "test value");

        unsafe {
            remove_var("TEST_KEY");
        }
    }
}
