use crate::{BaseDataModel, TenantId, UserRole};
use watchmen_model_marco::{adapt_model, Display, Serde};

#[derive(Display, Serde)]
pub enum TokenType {
    Bearer,
}

#[adapt_model(bdm)]
pub struct Token {
    pub access_token: Option<String>,
    pub token_type: Option<TokenType>,
    pub role: Option<UserRole>,
    pub tenant_id: Option<TenantId>,
}

// TODO for derived token structs, how to handle serde things?
