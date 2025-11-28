use crate::{ArcParameterJoint, RuntimeModelKernelErrorCode};
use std::sync::Arc;
use watchmen_model::{FactorId, ParameterJoint, StdErrorCode, StdR, StringUtils, TopicId};

pub trait ArcHelper {
    fn arc<V>(value: Option<V>) -> Option<Arc<V>> {
        value.map(Arc::new)
    }

    /// use the value or use default value [""] when there is no value given
    fn or_empty_str(value: Option<String>) -> Arc<String> {
        Arc::new(value.unwrap_or("".to_string()))
    }

    fn must<V, F>(value: Option<V>, on_none: F) -> StdR<Arc<V>>
    where
        F: FnOnce() -> StdR<Arc<V>>,
    {
        if value.is_none() {
            on_none()
        } else {
            Ok(Arc::new(value.unwrap()))
        }
    }

    /// check topic id is not missing and not blank
    fn topic_id<F, P>(topic_id: Option<TopicId>, pos: F) -> StdR<Arc<TopicId>>
    where
        F: Fn() -> P,
        P: Into<String>,
    {
        Self::not_blank(
            topic_id,
            || {
                RuntimeModelKernelErrorCode::TopicIdMissed
                    .msg(format!("{} must have a topic id.", pos().into()))
            },
            || {
                RuntimeModelKernelErrorCode::TopicIdIsBlank
                    .msg(format!("{} cannot be blank.", pos().into()))
            },
        )
    }

    /// check factor id is not missing and not blank
    fn factor_id<F, P>(factor_id: Option<FactorId>, pos: F) -> StdR<Arc<FactorId>>
    where
        F: Fn() -> P,
        P: Into<String>,
    {
        Self::not_blank(
            factor_id,
            || {
                RuntimeModelKernelErrorCode::FactorIdMissed
                    .msg(format!("{} must have a factor id.", pos().into()))
            },
            || {
                RuntimeModelKernelErrorCode::FactorIdIsBlank
                    .msg(format!("{} cannot be blank.", pos().into()))
            },
        )
    }

    /// check the given value:
    /// - empty: call [on_empty],
    /// - blank: call [on_blank],
    /// or return [Arc] if pass the check.
    fn not_blank<F1, F2>(value: Option<String>, on_empty: F1, on_blank: F2) -> StdR<Arc<String>>
    where
        F1: FnOnce() -> StdR<Arc<String>>,
        F2: FnOnce() -> StdR<Arc<String>>,
    {
        if let Some(v) = value {
            if v.is_not_blank() {
                Ok(Arc::new(v))
            } else {
                on_blank()
            }
        } else {
            on_empty()
        }
    }

    /// check the given args:
    /// - [conditional] use default value [false] if given arg not presents,
    /// - [on] cannot be empty if [conditional] is [true]
    fn conditional<F>(
        conditional: Option<bool>,
        on: Option<ParameterJoint>,
        msg: F,
    ) -> StdR<Option<Arc<ArcParameterJoint>>>
    where
        F: FnOnce() -> String,
    {
        let conditional = conditional.unwrap_or(false);
        if conditional {
            if on.is_none() {
                RuntimeModelKernelErrorCode::ConditionMissed.msg(msg())
            } else {
                Ok(Some(ArcParameterJoint::new(on.unwrap())?))
            }
        } else {
            Ok(None)
        }
    }
}
