use crate::{bdm, FactorId, TopicId};

pub struct TopicFactorParameter {
    pub topic_id: Option<TopicId>,
    pub factor_id: Option<FactorId>,
}

bdm!(TopicFactorParameter);
