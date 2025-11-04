use super::{parameter::Parameter, parameter_kind::ParameterKind};
use crate::serde_for_enum;
use std::fmt;

pub enum ParameterComputeType {
    None,
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulus,
    YearOf,
    HalfYearOf,
    QuarterOf,
    MonthOf,
    WeekOfYear,
    WeekOfMonth,
    DayOfMonth,
    DayOfWeek,
    CaseThen,
}

impl fmt::Display for ParameterComputeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParameterComputeType::None => write!(f, "none"),
            ParameterComputeType::Add => write!(f, "add"),
            ParameterComputeType::Subtract => write!(f, "subtract"),
            ParameterComputeType::Multiply => write!(f, "multiply"),
            ParameterComputeType::Divide => write!(f, "divide"),
            ParameterComputeType::Modulus => write!(f, "modulus"),
            ParameterComputeType::YearOf => write!(f, "year-of"),
            ParameterComputeType::HalfYearOf => write!(f, "half-year-of"),
            ParameterComputeType::QuarterOf => write!(f, "quarter-of"),
            ParameterComputeType::MonthOf => write!(f, "month-of"),
            ParameterComputeType::WeekOfYear => write!(f, "week-of-year"),
            ParameterComputeType::WeekOfMonth => write!(f, "week-of-month"),
            ParameterComputeType::DayOfMonth => write!(f, "day-of-month"),
            ParameterComputeType::DayOfWeek => write!(f, "day-of-week"),
            ParameterComputeType::CaseThen => write!(f, "case-then"),
        }
    }
}

serde_for_enum! {
    ParameterComputeType {
        None => "none",
        Add => "add",
        Subtract => "subtract",
        Multiply => "multiply",
        Divide => "divide",
        Modulus => "modulus",
        YearOf => "year-of",
        HalfYearOf => "half-year-of",
        QuarterOf => "quarter-of",
        MonthOf => "month-of",
        WeekOfYear => "week-of-year",
        WeekOfMonth => "week-of-month",
        DayOfMonth => "day-of-month",
        DayOfWeek => "day-of-week",
        CaseThen => "case-then",
    }
}

pub trait ComputedParameter: Parameter {
    fn kind(&self) -> Option<ParameterKind> {
        Some(ParameterKind::Computed)
    }
    fn compute_type(&self) -> Option<ParameterComputeType>;
    fn parameters(&self) -> Option<Vec<Box<dyn Parameter>>>;
}
