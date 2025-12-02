use crate::PipelineExecutionContext;
use watchmen_model::VoidR;

pub struct PipelineExecutor {}

impl PipelineExecutor {
    pub fn execute(_context: PipelineExecutionContext) -> VoidR {
        todo!("implement execute for PipelineExecutor")
    }

    pub async fn execute_async(_context: PipelineExecutionContext) -> VoidR {
        todo!("implement execute for PipelineExecutor")
    }
}
