use crate::{Factor, FactorIndexGroup, FactorType, Topic, TopicKind, TopicType};

pub fn ask_dqc_rule_daily_topic() -> Topic {
    let mut factors = Vec::new();
    factors.push(
        Factor::new()
            .factor_id(String::from("dra-f-1"))
            .name(String::from("ruleCode"))
            .r#type(FactorType::Text)
            .index_group(FactorIndexGroup::Index1),
    );
    factors.push(
        Factor::new()
            .factor_id(String::from("dra-f-2"))
            .name(String::from("topicId"))
            .r#type(FactorType::Text)
            .index_group(FactorIndexGroup::Index2),
    );
    factors.push(
        Factor::new()
            .factor_id(String::from("dra-f-3"))
            .name(String::from("factorId"))
            .r#type(FactorType::Text)
            .index_group(FactorIndexGroup::Index3),
    );
    factors.push(
        Factor::new()
            .factor_id(String::from("dra-f-4"))
            .name(String::from("year"))
            .r#type(FactorType::Year)
            .index_group(FactorIndexGroup::Index4),
    );
    factors.push(
        Factor::new()
            .factor_id(String::from("dra-f-5"))
            .name(String::from("month"))
            .r#type(FactorType::Month)
            .index_group(FactorIndexGroup::Index5),
    );
    factors.push(
        Factor::new()
            .factor_id(String::from("dra-f-6"))
            .name(String::from("day"))
            .r#type(FactorType::DayOfMonth)
            .index_group(FactorIndexGroup::Index6),
    );
    factors.push(
        Factor::new()
            .factor_id(String::from("dra-f-7"))
            .name(String::from("processDate"))
            .r#type(FactorType::Date)
            .index_group(FactorIndexGroup::Index7),
    );
    // the start day of date range
    // sunday of weekly; 1st of monthly.
    factors.push(
        Factor::new()
            .factor_id(String::from("dra-f-8"))
            .name(String::from("count"))
            .r#type(FactorType::Unsigned)
            .precision(String::from("10")),
    );

    Topic::new()
        .name(String::from("dqc_rule_daily"))
        .kind(TopicKind::System)
        .r#type(TopicType::Distinct)
        .factors(factors)
        .description(String::from("Topic data monitor by rules, distinct topic"))
        .build()
}
