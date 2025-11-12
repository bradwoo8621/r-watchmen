pub enum TopicKind {
    System,
}
pub enum TopicType {
    Raw,
}
pub struct Topic {
    pub name: &'static str,
    pub kind: TopicKind,
    pub r#type: TopicType,
    pub factors: Vec<Factor>,
    pub description: &'static str,
}

pub enum FactorType {
    Text,
    Number,
}
pub enum FactorIndexGroup {
    Index1,
    Index2,
}
pub struct Factor {
    pub factor_id: &'static str,
    pub name: &'static str,
    pub r#type: FactorType,
    pub flatten: bool,
    pub index_group: Option<FactorIndexGroup>,
    pub precision: Option<&'static str>,
}

pub struct Pipeline {}

pub fn create_qpt_topic() -> Topic {
    let mut factors = Vec::new();
    factors.push(Factor {
        factor_id: "rmpl-f-1",
        name: "uid",
        r#type: FactorType::Text,
        flatten: false,
        index_group: None,
        precision: None,
    });
    factors.push(Factor {
        factor_id: "rmpl-f-2",
        name: "topic_dimensions",
        r#type: FactorType::Text,
        flatten: true,
        index_group: Some(FactorIndexGroup::Index1),
        precision: Some("200"),
    });
    factors.push(Factor {
        factor_id: "rmpl-f-3",
        name: "column_dimensions",
        r#type: FactorType::Text,
        flatten: true,
        index_group: Some(FactorIndexGroup::Index2),
        precision: Some("200"),
    });
    factors.push(Factor {
        factor_id: "rmpl-f-4",
        name: "execution_time",
        r#type: FactorType::Number,
        flatten: false,
        index_group: None,
        precision: Some("50"),
    });
    factors.push(Factor {
        factor_id: "rmpl-f-5",
        name: "data_volume",
        r#type: FactorType::Number,
        flatten: false,
        index_group: None,
        precision: Some("50"),
    });
    factors.push(Factor {
        factor_id: "rmpl-f-6",
        name: "join_dimensions",
        r#type: FactorType::Text,
        flatten: true,
        index_group: None,
        precision: Some("200"),
    });
    factors.push(Factor {
        factor_id: "rmpl-f-7",
        name: "where_dimensions",
        r#type: FactorType::Text,
        flatten: true,
        index_group: None,
        precision: Some("200"),
    });
    factors.push(Factor {
        factor_id: "rmpl-f-8",
        name: "group_by_dimensions",
        r#type: FactorType::Text,
        flatten: true,
        index_group: None,
        precision: Some("200"),
    });

    Topic {
        name: "query_performance_log",
        kind: TopicKind::System,
        r#type: TopicType::Raw,
        factors,
        description: "query performance	log	raw	topic",
    }
}

pub fn ask_query_performance_topics() -> Vec<Topic> {
    let mut topics = Vec::new();
    topics.push(create_qpt_topic());
    topics
}

pub fn ask_query_performance_pipelines(_topics: Vec<Topic>) -> Vec<Pipeline> {
    // TODO define all pipeline monitor pipelines
    Vec::new()
}
