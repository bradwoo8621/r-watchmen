use crate::serde::option_naive_datetime;
use crate::{
    Auditable, BaseDataModel, DataSourceId, Factor, OptimisticLock, Storable, TenantBasedTuple,
    TenantId, Tuple, UserId,
};
use chrono::NaiveDateTime;
use watchmen_model_marco::{adapt_model, Display, Serde};

#[derive(Display, Serde)]
pub enum TopicKind {
    System,
    Business,
    Synonym,
}

#[derive(Display, Serde)]
pub enum TopicType {
    Raw,
    Meta,
    Distinct,
    Aggregate,
    Time,
    Ratio,
}

pub type TopicId = String;

#[adapt_model(opt_lock, tenant_based)]
pub struct Topic {
    pub topic_id: Option<TopicId>,
    pub name: Option<String>,
    pub r#type: Option<TopicType>,
    pub kind: Option<TopicKind>,
    pub data_source_id: Option<DataSourceId>,
    pub factors: Option<Vec<Factor>>,
    pub description: Option<String>,
}

pub struct TopicBuilder {
    topic_id: Option<TopicId>,
    name: Option<String>,
    r#type: Option<TopicType>,
    kind: Option<TopicKind>,
    data_source_id: Option<DataSourceId>,
    factors: Option<Vec<Factor>>,
    description: Option<String>,
    tenant_id: Option<TenantId>,
    version: Option<u32>,
    created_at: Option<NaiveDateTime>,
    created_by: Option<UserId>,
    last_modified_at: Option<NaiveDateTime>,
    last_modified_by: Option<UserId>,
}

impl TopicBuilder {
    pub fn new() -> Self {
        TopicBuilder {
            topic_id: None,
            name: None,
            r#type: None,
            kind: None,
            data_source_id: None,
            factors: None,
            description: None,
            tenant_id: None,
            version: None,
            created_at: None,
            created_by: None,
            last_modified_at: None,
            last_modified_by: None,
        }
    }

    pub fn topic_id(mut self, topic_id: TopicId) -> Self {
        self.topic_id = Some(topic_id);
        self
    }
    
    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    pub fn r#type(mut self, r#type: TopicType) -> Self {
        self.r#type = Some(r#type);
        self
    }

    pub fn kind(mut self, kind: TopicKind) -> Self {
        self.kind = Some(kind);
        self
    }

    pub fn data_source_id(mut self, data_source_id: DataSourceId) -> Self {
        self.data_source_id = Some(data_source_id);
        self
    }

    pub fn factors(mut self, factors: Vec<Factor>) -> Self {
        self.factors = Some(factors);
        self
    }

    pub fn description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    pub fn tenant_id(mut self, tenant_id: TenantId) -> Self {
        self.tenant_id = Some(tenant_id);
        self
    }

    pub fn build(self) -> Topic {
        Topic {
            topic_id: self.topic_id,
            name: self.name,
            r#type: self.r#type,
            kind: self.kind,
            data_source_id: self.data_source_id,
            factors: self.factors,
            description: self.description,
            tenant_id: self.tenant_id,
            version: self.version,
            created_at: self.created_at,
            created_by: self.created_by,
            last_modified_at: self.last_modified_at,
            last_modified_by: self.last_modified_by,
        }
    }
}
