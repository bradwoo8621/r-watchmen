use crate::PathParser;
use watchmen_model::{StdR, VariablePredefineFunctions};

impl PathParser<'_> {
    /// path is empty
    pub fn incorrect_empty_path<R>(&self) -> StdR<R> {
        self.inner
            .error("Incorrect data path, caused by not content determined.")
    }

    /// report error at [index of blank string start, current char index)
    pub fn incorrect_blank_segment<R>(&self) -> StdR<R> {
        let inner = &self.inner;
        inner.error(format!(
            "Incorrect data path[{}], caused by blank segment at index[{}, {}].",
            inner.full_path,
            inner.char_index - inner.in_memory_chars_count(),
            inner.char_index,
        ))
    }

    /// report error at [index of left brace]
    pub fn incorrect_wrapped_path<R>(&self, index_of_left_brace: usize) -> StdR<R> {
        let inner = &self.inner;
        inner.error(format!(
            "Incorrect data path[{}], caused by the closing \"}}\" is not matched, the opening \"{{\" is at index [{}].",
            inner.full_path, index_of_left_brace
        ))
    }

    /// report error at [current char index]
    pub fn incorrect_function_name_char<R>(&self, char: char) -> StdR<R> {
        let inner = &self.inner;
        inner.error(format!(
            "Incorrect data path[{}], caused by disallowed char[{}] in function name at index[{}].",
            inner.full_path, char, inner.char_index
        ))
    }

    /// report error at [current char index - 1]
    pub fn incorrect_empty_function_name<R>(&self) -> StdR<R> {
        let inner = &self.inner;
        inner.error(format!(
            "Incorrect data path[{}], caused by empty function name at index[{}].",
            inner.full_path,
            inner.char_index - 1
        ))
    }

    /// report error at [current char index - in memory chars count, current char index)
    pub fn incorrect_function_name<R>(&self, in_memory_chars_count: usize) -> StdR<R> {
        let inner = &self.inner;
        let end_char_index = inner.char_index;
        let start_char_index = end_char_index - in_memory_chars_count;
        inner.error(format!(
            "Incorrect data path[{}], caused by unrecognized function name[{}] at index[{}, {}].",
            inner.full_path,
            inner.full_path[start_char_index..end_char_index].to_string(),
            start_char_index,
            end_char_index
        ))
    }

    /// report error at [index of ampersand, current char index + 1)
    pub fn incorrect_function_name_contains_whitespace<R>(
        &self,
        index_of_ampersand: usize,
    ) -> StdR<R> {
        let inner = &self.inner;
        let end_char_index = inner.char_index + 1;
        inner.error(format!(
            "Incorrect data path[{}], caused by function name[{}] contains whitespace(s) at index[{}, {}].",
            inner.full_path,
            inner.full_path[index_of_ampersand..end_char_index].to_string(),
            index_of_ampersand,
            end_char_index
        ))
    }

    /// report error at [index of ampersand, current char index)
    pub fn incorrect_function_no_param<R>(&self, func: &VariablePredefineFunctions) -> StdR<R> {
        let inner = &self.inner;
        let end_char_index = inner.char_index;
        let start_char_index = end_char_index - func.to_string().chars().count();
        inner.error(format!(
            "Incorrect data path[{}], caused by function[{}] must have parameter(s) at index[{}, {}].",
            inner.full_path,
            inner.full_path[start_char_index..end_char_index].to_string(),
            start_char_index,
            end_char_index
        ))
    }

    /// report error at [index of ampersand, current char index)
    pub fn incorrect_function_has_context<R>(&self, index_of_ampersand: usize) -> StdR<R> {
        let inner = &self.inner;
        let end_char_index = inner.char_index;
        inner.error(format!(
            "Incorrect data path[{}], caused by function[{}] cannot have context at index[{}, {}].",
            inner.full_path,
            inner.full_path[index_of_ampersand..end_char_index].to_string(),
            index_of_ampersand,
            end_char_index
        ))
    }
}
