use crate::serde::option_naive_datetime;
use crate::{
    AIModelId, Auditable, BaseDataModel, OptimisticLock, Storable, TenantBasedTuple, TenantId,
    Tuple, UserId,
};
use watchmen_model_marco::{adapt_model, Display, Serde};

#[derive(Display, Serde)]
#[pattern = "keep-same"]
pub enum AiModelProvider {
    Anthropic,
    Microsoft,
    OpenAI,
    AWS,
    #[display = "Hugging Face"]
    HuggingFace,
    Google,
    Ollama,
}

#[adapt_model(opt_lock, tenant_based)]
pub struct AIModel {
    pub model_id: Option<AIModelId>,
    pub enable_monitor: Option<bool>,
    pub llm_provider: Option<AiModelProvider>,
    pub base_url: Option<String>,
    pub model_name: Option<String>,
    pub model_version: Option<String>,
    pub model_token: Option<String>,
    pub embedding_provider: Option<AiModelProvider>,
    pub base_embedding_url: Option<String>,
    pub embedding_name: Option<String>,
    pub embedding_version: Option<String>,
    pub embedding_token: Option<String>,
}
