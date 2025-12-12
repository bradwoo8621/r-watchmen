use crate::{
    BaseDataModel, CharsMatch, Parameter, ParameterKind, StdErrCode, StdErrorCode, StdR, Storable,
};
use std::clone::Clone;
use std::sync::{Arc, LazyLock};
use watchmen_model_marco::{adapt_model, Display, Serde, StrEnum};

#[derive(Display, Serde, StrEnum)]
#[pattern = "ampersand-prefix"]
#[chars_match]
pub enum VariablePredefineFunctions {
    // Sequence functions
    /// get next sequence number, [only in-memory]
    NextSeq,
    // Aggregation functions
    /// count of vec or map, [only in-memory]
    Count,
    // String functions
    /// chars count of string or decimal (to string)
    Length,
    /// alias of [Length]
    Len,
    Slice,
    /// alias of [Slice]
    Substr,
    Find,
    /// alias of [Find]
    Index,
    StartsWith,
    /// alias of [StartsWith]
    #[display = "&startswith"]
    Startswith,
    EndsWith,
    /// alias of [EndsWith]
    #[display = "&endswith"]
    Endswith,
    Strip,
    /// alias of [Strip]
    Trim,
    Replace,
    ReplaceFirst,
    Upper,
    Lower,
    Contains,
    Split,
    /// join the elements of vec to a string, [only in-memory]
    Join,
    // Statistical functions
    /// get a distinct vec, [only in-memory]
    Distinct,
    /// sum of elements of vec, [only in-memory]
    Sum,
    /// avg of elements of vec, [only in-memory]
    Avg,
    /// max of elements of vec, [only in-memory]
    Max,
    /// max decimal elements of vec, [only in-memory]
    MaxNum,
    /// max date of elements of vec, [only in-memory]
    MaxDate,
    /// max date time of elements of vec, [only in-memory]
    MaxDatetime,
    /// alias of [MaxDatetime]
    MaxDt,
    /// max time of elements of vec, [only in-memory]
    MaxTime,
    /// min of elements of vec, [only in-memory]
    Min,
    /// min decimal elements of vec, [only in-memory]
    MinNum,
    /// min date of elements of vec, [only in-memory]
    MinDate,
    /// min date time of elements of vec, [only in-memory]
    MinDatetime,
    /// alias of [MinDatetime]
    MinDt,
    /// min time of elements of vec, [only in-memory]
    MinTime,
    // Retrieve value from previous trigger data
    #[display = "&old"]
    FromPreviousTriggerData,
    // Date related functions
    DayDiff,
    MonthDiff,
    YearDiff,
    MoveDate,
    #[display = "&fmtDate"]
    DateFormat,
    Now,
}

pub static VARIABLE_PREDEFINE_FUNCTIONS_CHARS_MATCH: LazyLock<Arc<CharsMatch>> =
    LazyLock::new(|| {
        Arc::new(
            VariablePredefineFunctions::create_chars_match()
                .of(&'&')
                .clone(),
        )
    });

/// string stands for an expression to retrieve some value
/// might include function calls, see [VariablePredefineFunctions]
#[adapt_model(storable)]
pub struct ConstantParameter {
    pub kind: Option<ParameterKind>,
    pub value: Option<String>,
}

impl ConstantParameter {
    pub fn init() -> Self {
        Self::new().kind(ParameterKind::Constant)
    }

    pub fn of(value: String) -> Self {
        Self::init().value(value)
    }

    pub fn to_parameter(self) -> Parameter {
        Parameter::Constant(self)
    }
}

#[cfg(test)]
mod tests {
    use crate::VARIABLE_PREDEFINE_FUNCTIONS_CHARS_MATCH;

    #[test]
    fn test() {
        let chars_match = &VARIABLE_PREDEFINE_FUNCTIONS_CHARS_MATCH;
        assert!(chars_match.matches(&vec!['n', 'o']));
        assert!(chars_match.matches(&vec!['n', 'o', 'w']));
        assert_eq!(chars_match.matches(&vec!['n', 'o', 'w', '1']), false);
    }
}
