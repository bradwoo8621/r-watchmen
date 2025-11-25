use crate::{
    ArcTopic, TopicSchemaDefaultValueFactorGroup, TopicSchemaDefaultValueFactorGroups,
    TopicSchemaFactorGroups,
};
use std::sync::Arc;
use watchmen_model::Topic;

/// The schema of a topic, including various factor groups.
/// all factor fields are optional, depending on whether the topic has the corresponding factors.
pub struct TopicSchema {
    topic: Arc<ArcTopic>,
    _flatten_factors: Option<String>, // Option<Arc<Vec<Arc<TopicSchemaFlattenFactor>>>>,
    _date_or_time_factors: Option<String>, // Option<Arc<Vec<Arc<TopicSchemaDateOrTimeFactor>>>>,
    _encrypt_factor_groups: Option<String>, // Option<Arc<Vec<Arc<TopicSchemaEncryptFactor>>>>,
    default_value_factor_groups: Option<Arc<Vec<Arc<TopicSchemaDefaultValueFactorGroup>>>>,
}

impl TopicSchema {
    pub fn new(topic: Topic) -> Self {
        let arc_topic = ArcTopic::from(topic);
        TopicSchema {
            topic: arc_topic.clone(),
            _flatten_factors: None,
            _date_or_time_factors: None,
            _encrypt_factor_groups: None,
            default_value_factor_groups: TopicSchemaDefaultValueFactorGroups::create(&arc_topic),
        }
    }

    pub fn topic(&self) -> &Arc<ArcTopic> {
        &self.topic
    }
}
