use crate::Topic;
use crate::topics::ask_query_performance_log_topic;

pub fn ask_query_performance_topics() -> Vec<Topic> {
	let mut topics = Vec::new();
	topics.push(ask_query_performance_log_topic());
	topics
}
