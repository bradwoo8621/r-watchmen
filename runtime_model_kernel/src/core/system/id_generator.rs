use crate::RuntimeModelKernelErrorCode;
use std::sync::{Arc, OnceLock, RwLock};
use watchmen_model::{StdErrorCode, VoidR};

/// max to 20 digits numbers
pub trait IdGenerator: Send + Sync {
    fn next_id(&self) -> u128;
}

pub struct DummyIdGenerator();

impl IdGenerator for DummyIdGenerator {
    fn next_id(&self) -> u128 {
        panic!("DummyIdGenerator does not support any ID generator")
    }
}

static GLOBAL_ID_GENERATOR: OnceLock<Arc<RwLock<Box<dyn IdGenerator>>>> = OnceLock::new();

pub struct IdGen();

impl IdGen {
    pub fn get() -> Arc<RwLock<Box<dyn IdGenerator>>> {
        GLOBAL_ID_GENERATOR
            .get_or_init(|| {
                let generator = Box::new(DummyIdGenerator()) as Box<dyn IdGenerator>;
                Arc::new(RwLock::new(generator))
            })
            .clone()
    }

    pub fn set(new_generator: Box<dyn IdGenerator>) -> VoidR {
        let old_arc = GLOBAL_ID_GENERATOR.get_or_init(|| {
            Arc::new(RwLock::new(
                Box::new(DummyIdGenerator()) as Box<dyn IdGenerator>
            ))
        });
        match old_arc.write() {
            Ok(mut guard) => {
                *guard = new_generator;
                Ok(())
            }
            Err(e) => RuntimeModelKernelErrorCode::CannotSetIdGenerator.msg(e.to_string()),
        }
    }
}
