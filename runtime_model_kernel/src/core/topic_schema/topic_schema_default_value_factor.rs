use crate::{
    ArcFactor, TopicSchemaFactor, TopicSchemaFactorGroup, TopicSchemaFactorGroupInner,
    TopicSchemaFactorGroups, TopicSchemaFactorInner, TopicSchemaGroupFactor,
};
use std::sync::Arc;
use watchmen_model::TopicDataValue;

pub struct TopicSchemaDefaultValueFactor {
    inner: TopicSchemaFactorInner,
    default_value: Option<Arc<TopicDataValue>>,
}

impl TopicSchemaDefaultValueFactor {
    pub fn new(inner: TopicSchemaFactorInner, default_value: Option<Arc<TopicDataValue>>) -> Self {
        TopicSchemaDefaultValueFactor {
            inner,
            default_value,
        }
    }

    pub fn default_value(&self) -> &Option<Arc<TopicDataValue>> {
        &self.default_value
    }
}

impl TopicSchemaFactor for TopicSchemaDefaultValueFactor {
    fn get_inner(&self) -> &TopicSchemaFactorInner {
        &self.inner
    }
}

impl TopicSchemaGroupFactor<TopicSchemaDefaultValueFactor> for TopicSchemaDefaultValueFactor {
    fn replace_names(&self, names: Arc<Vec<String>>) -> TopicSchemaDefaultValueFactor {
        TopicSchemaDefaultValueFactor {
            inner: self.get_inner().replace_names(names),
            default_value: self.default_value.clone(),
        }
    }
}

pub type TopicSchemaDefaultValueFactorGroupInner =
    TopicSchemaFactorGroupInner<TopicSchemaDefaultValueFactor, TopicSchemaDefaultValueFactorGroup>;

pub struct TopicSchemaDefaultValueFactorGroup {
    inner: TopicSchemaDefaultValueFactorGroupInner,
}

impl TopicSchemaDefaultValueFactorGroup {
    pub fn new(inner: TopicSchemaDefaultValueFactorGroupInner) -> Self {
        TopicSchemaDefaultValueFactorGroup { inner }
    }
}

impl TopicSchemaFactorGroup<'_, TopicSchemaDefaultValueFactor, TopicSchemaDefaultValueFactorGroup>
    for TopicSchemaDefaultValueFactorGroup
{
    type Inner = TopicSchemaDefaultValueFactorGroupInner;

    fn new(name: Arc<String>, factors: Arc<Vec<Arc<TopicSchemaDefaultValueFactor>>>) -> Self {
        TopicSchemaDefaultValueFactorGroup::new(TopicSchemaFactorGroupInner::new(name, factors))
    }

    fn get_inner(&self) -> &TopicSchemaDefaultValueFactorGroupInner {
        &self.inner
    }
}

pub struct TopicSchemaDefaultValueFactorGroups;

impl TopicSchemaFactorGroups<TopicSchemaDefaultValueFactor, TopicSchemaDefaultValueFactorGroup>
    for TopicSchemaDefaultValueFactorGroups
{
    fn accept_factor(factor: &Arc<ArcFactor>) -> bool {
        factor.has_default_value()
    }

    fn create_schema_factor(factor: &Arc<ArcFactor>) -> TopicSchemaDefaultValueFactor {
        TopicSchemaDefaultValueFactor {
            inner: TopicSchemaFactorInner::new(factor.clone()),
            default_value: None,
        }
    }

    fn create_schema_group(
        name: String,
        factors: Arc<Vec<Arc<TopicSchemaDefaultValueFactor>>>,
    ) -> TopicSchemaDefaultValueFactorGroup {
        TopicSchemaDefaultValueFactorGroup::new(TopicSchemaDefaultValueFactorGroupInner::new(
            Arc::new(name),
            factors,
        ))
    }
}
