use crate::{Factor, FactorIndexGroup, FactorType, Topic, TopicKind, TopicType};

pub fn ask_dqc_raw_rule_result_topic() -> Topic {
    let mut factors = Vec::new();
    factors.push(
        Factor::new()
            .factor_id(String::from("dra-f-1"))
            .name(String::from("ruleCode"))
            .r#type(FactorType::Text)
            .flatten(true)
            .index_group(FactorIndexGroup::Index1)
            .precision(String::from("50"))
            .build(),
    );
    factors.push(
        Factor::new()
            .factor_id(String::from("dra-f-2"))
            .name(String::from("topicId"))
            .r#type(FactorType::Text)
            .flatten(true)
            .index_group(FactorIndexGroup::Index2)
            .precision(String::from("200"))
            .build(),
    );
    factors.push(
        Factor::new()
            .factor_id(String::from("dra-f-3"))
            .name(String::from("topicName"))
            .r#type(FactorType::Text)
            .build(),
    );
    factors.push(
        Factor::new()
            .factor_id(String::from("dra-f-4"))
            .name(String::from("factorId"))
            .r#type(FactorType::Text)
            .flatten(true)
            .index_group(FactorIndexGroup::Index3)
            .precision(String::from("50"))
            .build(),
    );
    factors.push(
        Factor::new()
            .factor_id(String::from("dra-f-5"))
            .name(String::from("factorName"))
            .r#type(FactorType::Text)
            .build(),
    );
    factors.push(
        Factor::new()
            .factor_id(String::from("dra-f-6"))
            .name(String::from("detected"))
            .r#type(FactorType::Boolean)
            .flatten(true)
            .build(),
    );
    factors.push(
        Factor::new()
            .factor_id(String::from("dra-f-7"))
            .name(String::from("severity"))
            .r#type(FactorType::Text)
            .build(),
    );
    // the start day of date range
    // sunday of weekly; 1st of monthly.
    factors.push(
        Factor::new()
            .factor_id(String::from("dra-f-8"))
            .name(String::from("processDate"))
            .r#type(FactorType::Date)
            .flatten(true)
            .index_group(FactorIndexGroup::Index4)
            .build(),
    );

    Topic::new()
        .name(String::from("dqc_raw_rule_result"))
        .kind(TopicKind::System)
        .r#type(TopicType::Raw)
        .factors(factors)
        .description(String::from("Topic data monitor by rules, raw topic"))
        .build()
}
