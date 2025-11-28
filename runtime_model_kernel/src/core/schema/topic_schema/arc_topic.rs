use crate::{ArcFactor, ArcHelper, RuntimeModelKernelErrorCode};
use std::sync::Arc;
use watchmen_model::{
    DataSourceId, StdErrorCode, StdR, TenantId, Topic, TopicCode, TopicId, TopicKind, TopicType,
};

/// An Arc-wrapped version of Topic for shared ownership and thread safety.
/// This struct uses Arc pointers for each field to allow efficient sharing
/// of data across multiple threads without unnecessary cloning.
///
/// fields are same as [Topic], description and audit columns are omitted.
#[derive(Debug)]
pub struct ArcTopic {
    pub topic_id: Arc<TopicId>,
    pub name: Arc<TopicCode>,
    pub r#type: Arc<TopicType>,
    pub kind: Arc<TopicKind>,
    pub data_source_id: Option<Arc<DataSourceId>>,
    pub factors: Arc<Vec<Arc<ArcFactor>>>,
    pub tenant_id: Arc<TenantId>,
    pub version: Option<u32>,
}

impl ArcHelper for ArcTopic {}

impl ArcTopic {
    pub fn new(topic: Topic) -> StdR<Arc<Self>> {
        let topic_id = Self::topic_id(topic.topic_id, || "Topic")?;

        let name = Self::not_blank(
            topic.name,
            || {
                RuntimeModelKernelErrorCode::TopicNameMissed
                    .msg(format!("Topic[{}] must have a name.", topic_id))
            },
            || {
                RuntimeModelKernelErrorCode::TopicNameIsBlank
                    .msg(format!("Topic[{}]'s name cannot be blank.", topic_id))
            },
        )?;
        // TODO
        if topic.tenant_id.is_none() {
            return RuntimeModelKernelErrorCode::TopicTenantMissed
                .msg(format!("Topic[{}] has not tenant.", name));
        }
        let tenant_id = Arc::new(topic.tenant_id.unwrap());

        let r#type = Self::must(topic.r#type, || {
            RuntimeModelKernelErrorCode::TopicTypeMissed
                .msg(format!("Topic[{}] must have a type.", topic_id))
        })?;
        let kind = Self::must(topic.kind, || {
            RuntimeModelKernelErrorCode::TopicKindMissed
                .msg(format!("Topic[{}] must have a kind.", topic_id))
        })?;

        if topic.factors.is_none() {
            return RuntimeModelKernelErrorCode::TopicFactorMissed
                .msg(format!("Topic[{}] has no factor.", name));
        }
        let factors = topic.factors.unwrap();
        if factors.len() == 0 {
            return RuntimeModelKernelErrorCode::TopicFactorMissed
                .msg(format!("Topic[{}] has no factor.", name));
        }
        let mut arc_factors = vec![];
        for factor in factors {
            arc_factors.push(ArcFactor::new(factor)?);
        }
        let arc_factors = Arc::new(arc_factors);

        Ok(Arc::new(Self {
            topic_id,
            name,
            r#type,
            kind,
            data_source_id: Self::arc(topic.data_source_id),
            factors: arc_factors,
            tenant_id,
            version: topic.version,
        }))
    }

    pub fn is_raw_topic(&self) -> bool {
        self.r#type.is_raw_topic()
    }

    pub fn is_meta_topic(&self) -> bool {
        self.r#type.is_meta_topic()
    }

    pub fn is_distinct_topic(&self) -> bool {
        self.r#type.is_distinct_topic()
    }

    pub fn is_aggregation_topic(&self) -> bool {
        self.r#type.is_aggregation_topic()
    }

    pub fn is_time_topic(&self) -> bool {
        self.r#type.is_time_topic()
    }

    pub fn is_ratio_topic(&self) -> bool {
        self.r#type.is_ratio_topic()
    }

    pub fn is_system_topic(&self) -> bool {
        self.kind.is_system()
    }

    pub fn is_business_topic(&self) -> bool {
        self.kind.is_business()
    }

    pub fn is_synonym_topic(&self) -> bool {
        self.kind.is_synonym()
    }
}
