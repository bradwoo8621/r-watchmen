use crate::criteria::parameter::Parameter;
use crate::criteria::parameter_condition::ParameterCondition;
use crate::serde_for_enum;
use std::fmt;

pub enum ParameterExpressionOperator {
    Empty,
    NotEmpty,
    Equal,
    NotEqual,
    LessThan,
    LessThanOrEqual,
    MoreThan,
    MoreThanOrEqual,
    In,
    NotIn,
}

impl fmt::Display for ParameterExpressionOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParameterExpressionOperator::Empty => write!(f, "empty"),
            ParameterExpressionOperator::NotEmpty => write!(f, "not-empty"),
            ParameterExpressionOperator::Equal => write!(f, "equals"),
            ParameterExpressionOperator::NotEqual => write!(f, "not-equals"),
            ParameterExpressionOperator::LessThan => write!(f, "less"),
            ParameterExpressionOperator::LessThanOrEqual => write!(f, "less-equals"),
            ParameterExpressionOperator::MoreThan => write!(f, "more"),
            ParameterExpressionOperator::MoreThanOrEqual => write!(f, "more-equals"),
            ParameterExpressionOperator::In => write!(f, "in"),
            ParameterExpressionOperator::NotIn => write!(f, "not-in "),
        }
    }
}

serde_for_enum! {
    ParameterExpressionOperator {
        Empty => "empty",
        NotEmpty => "not-empty",
        Equal => "equals",
        NotEqual => "not-equals",
        LessThan => "less",
        LessThanOrEqual => "less-equals",
        MoreThan => "more",
        MoreThanOrEqual => "more-equals",
        In => "in",
        NotIn => "not-in ",
    }
}

pub trait ParameterExpression: ParameterCondition {
    fn left(&self) -> Option<Box<dyn Parameter>>;
    fn operator(&self) -> Option<ParameterExpressionOperator>;
    fn right(&self) -> Option<Box<dyn Parameter>>;
}
