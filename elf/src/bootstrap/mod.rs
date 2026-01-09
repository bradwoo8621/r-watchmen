use watchmen_base::{DateTimeFormatterInitializer, EnvConfig, VoidR};

pub struct ElfBootstrap;

impl ElfBootstrap {
    pub fn init_envs(env_config: &EnvConfig) -> VoidR {
        DateTimeFormatterInitializer::init(env_config)?;

        Ok(())
    }
}
