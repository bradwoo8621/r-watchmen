use crate::FuncParser;
use watchmen_model::StdR;

impl FuncParser<'_> {
    /// report error at [index of ampersand, current char index)
    pub fn incorrect_function_has_context<R>(&self) -> StdR<R> {
        let inner = &self.inner;
        let start_char_index = self.start_char_index_of_func;
        let end_char_index = inner.char_index;
        inner.error(format!(
            "Incorrect data path[{}], caused by function[{}] cannot have context at index[{}, {}].",
            inner.full_path,
            inner.full_path[start_char_index..end_char_index].to_string(),
            start_char_index,
            end_char_index
        ))
    }

    /// report error at [index of left parenthesis]
    pub fn incorrect_function_params_not_close<R>(
        &self,
        index_of_left_parenthesis: usize,
    ) -> StdR<R> {
        let inner = &self.inner;
        inner.error(format!(
            "Incorrect data path[{}], caused by the closing \")\" is not matched, the opening \"(\" is at index [{}].",
            inner.full_path, index_of_left_parenthesis
        ))
    }

    /// report error at [index of left parenthesis, current char index)
    pub fn incorrect_function_param_over_max_count<R>(
        &self,
        index_of_left_parenthesis: usize,
        max_count: usize,
    ) -> StdR<R> {
        let inner = &self.inner;
        inner.error(format!(
            "Incorrect data path[{}], caused by function[{}] can accept a maximum of {} parameters at index[{}, {}].",
            inner.full_path,
            self.func.to_string(),
            max_count,
            index_of_left_parenthesis,
            inner.char_index
        ))
    }

    /// report error at [current char index - in memory chars count, current char index)
    pub fn incorrect_function_param_tailing_whitespaces<R>(
        &self,
        in_memory_chars_count: usize,
    ) -> StdR<R> {
        let inner = &self.inner;
        let end_char_index = inner.char_index;
        let start_char_index = end_char_index - in_memory_chars_count;
        inner.error(format!(
            "Incorrect data path[{}], caused by function[{}] parameter has unexpected tailing whitespaces at index[{}, {}].",
            inner.full_path,
            self.func.to_string(),
            start_char_index,
            end_char_index
        ))
    }

    /// report error at [current char index - in memory chars count, current char index)
    pub fn incorrect_function_invalid_context<R>(&self, in_memory_chars_count: usize) -> StdR<R> {
        let inner = &self.inner;
        let end_char_index = inner.char_index;
        let start_char_index = end_char_index - in_memory_chars_count;
        inner.error(format!(
            "Incorrect data path[{}], caused by context of function[{}] is invalid at index[{}, {}].",
            inner.full_path,
            self.func.to_string(),
            start_char_index,
            end_char_index
        ))
    }
}
