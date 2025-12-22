use crate::{FuncDataPathParam, FuncParamValue, FuncParamValuePath, FuncParser};
use watchmen_model::StdR;

impl FuncParser<'_> {
    fn parse_param(&mut self) -> StdR<()> {
        // TODO
        Ok(())
    }

    fn check_tailing_whitespaces_when_param_parsed(&self) -> StdR<()> {
        if self.inner.in_memory_chars_is_not_empty() {
            // never happens, all chars should be consumed in parse_param
            self.incorrect_function_param_tailing_whitespaces(self.inner.in_memory_chars_count())
        } else {
            Ok(())
        }
    }

    /// consume in-memory chars as blank string parameter.
    /// and clear in-memory chars
    fn append_blank_param(&mut self) -> StdR<()> {
        self.params
            .push(FuncDataPathParam::Value(FuncParamValuePath {
                path: self.inner.clone_in_memory_chars(),
                value: FuncParamValue::Str(self.inner.clone_in_memory_chars()),
            }));

        self.inner.clear_in_memory_chars();

        Ok(())
    }

    /// consume in-memory chars as none parameter.
    /// and clear in-memory chars
    ///
    /// note the in-memory chars might not empty, it is treated as none here.
    fn append_none_param(&mut self) -> StdR<()> {
        self.params
            .push(FuncDataPathParam::Value(FuncParamValuePath {
                path: self.inner.clone_in_memory_chars(),
                value: FuncParamValue::None,
            }));

        self.inner.clear_in_memory_chars();

        Ok(())
    }

    fn end_param_at_0(&mut self) -> StdR<()> {
        let parsed_count = self.params.len();
        if parsed_count != 0 {
            return self.check_tailing_whitespaces_when_param_parsed();
        }

        if self.func.require_context() {
            // current is context
            if self.inner.in_memory_chars_is_not_empty() {
                // there are chars in memory
                if self.func.allow_blank_context() {
                    // context allows string
                    self.append_blank_param()
                } else if self.func.allow_none_context() {
                    self.append_none_param()
                } else {
                    // context does not allow none, raise error
                    self.incorrect_function_invalid_context(self.inner.in_memory_chars_count())
                }
            } else {
                // no char in memory, check none is acceptable or not
                if self.func.allow_none_context() {
                    self.append_none_param()
                } else {
                    // context does not allow none, raise error
                    self.incorrect_function_invalid_context(self.inner.in_memory_chars_count())
                }
            }
        } else {
            // current is parameter
            let max_param_count = self.func.max_param_count();
            if let Some(max_count) = max_param_count {
                if max_count == 0 {
                    // no parameter allowed, simply ignore
                    return Ok(());
                }
            }

            // TODO at least one parameter required, or no limit
            Ok(())
        }
    }

    /// end param parse when one of [,)] detected
    /// check parsed parameters count, if given param index already parsed (by [parse_param]),
    /// then do nothing.
    /// otherwise check the in-memory chars,
    /// - not empty,
    ///   - check function accept none in current param index or not
    ///   - check function accept blank string in current param index or not
    ///   - check function accept empty string in current param index or not
    ///   - none of above accepted, raise error (since blank plain path is not accepted).
    /// - empty,
    ///   - check function accept none in current param index or not
    ///   - check function accept empty string in current param index or not
    ///   - none of above accepted, raise error (since blank plain path is not accepted).
    /// move char index to next at last
    fn end_param(&mut self, param_index: usize) -> StdR<()> {
        let parsed_count = self.params.len();
        if param_index == 0 {
            // no parameter parsed, current is the first parameter.
            self.end_param_at_0()?
        } else if parsed_count < param_index {
        } else {
            // already parsed, do nothing
            self.check_tailing_whitespaces_when_param_parsed()?;
        }

        // move to next char, skip the ")" or ","
        self.inner.move_char_index_to_next();

        Ok(())
    }

    fn end_param_before_right_parenthesis(
        &mut self,
        index_of_left_parenthesis: usize,
        param_index: usize,
    ) -> StdR<()> {
        self.check_param_count_before_right_parenthesis(index_of_left_parenthesis, param_index)?;
        self.end_param(param_index)
    }

    /// it is not the last parameter of function, there is one more parameter will be parsed after comma.
    fn end_param_before_comma(
        &mut self,
        index_of_left_parenthesis: usize,
        param_index: usize,
    ) -> StdR<()> {
        self.check_param_count_before_comma(index_of_left_parenthesis, param_index)?;
        self.end_param(param_index)
    }

    fn finalize_content(&mut self) -> StdR<()> {
        // TODO
        Ok(())
    }

    /// now a function name and [(] encountered, so parameters part will end with a [)].
    ///
    /// the parameter can contain whitespaces,
    /// so it is hard to determine that the prefix/suffix whitespaces are valid or not.
    /// basically, current whitespaces are not allowed
    /// - before [&], e.g. [ &now] is invalid,
    /// - after [.], e.g. [a. ] is invalid,
    /// and suffix whitespaces in literal concat function will be collected as blank string value,
    /// it needs to be ignored.
    ///
    /// so when start parsing parameter, collect whitespaces into memory first,
    /// when there are in-memory whitespace chars collected, and
    /// - if there is a [&] determined, raise error,
    /// - if there is one of [,)] determined, means no parameter lacked, raise error,
    /// - if there is a [(] determined, raise error,
    /// - otherwise copy in-memory chars to path parser to continue.
    pub fn parse(&mut self) -> StdR<()> {
        let index_of_left_parenthesis = self.inner.char_index - 1;

        // total parsed parameter count, include context if not with context currently
        let mut param_index: usize = 0;
        let mut whitespace_met = true;
        loop {
            if let Some(char) = self.inner.current_char() {
                match char {
                    '&' => {
                        if whitespace_met {
                            self.inner.incorrect_ampersand()?;
                        } else {
                            // ignore the whitespaces before function
                            self.inner.clear_in_memory_chars();
                            self.parse_param()?;
                            whitespace_met = false;
                        }
                    }
                    '(' => self.inner.incorrect_left_parenthesis()?,
                    // end of parameters
                    ')' => {
                        self.end_param_before_right_parenthesis(
                            index_of_left_parenthesis,
                            param_index,
                        )?;
                        param_index += 1;
                        break;
                    }
                    // end of parameter
                    ',' => {
                        self.end_param_before_comma(index_of_left_parenthesis, param_index)?;
                        param_index += 1;
                    }
                    // \s, ignore.
                    // in python version, whitespaces after function name is allowed, compatible logic here
                    ' ' | '\t' | '\r' | '\n' | '\x0C' | '\x0B' => {
                        whitespace_met = true;
                        self.inner
                            .consume_char_into_memory_and_move_char_index_to_next(*char)
                    }
                    // other char
                    _ => {
                        self.parse_param()?;
                        whitespace_met = false;
                    }
                };
            } else {
                // reach the end, no char anymore
                // ")" not encountered, raise error
                return self.incorrect_function_params_not_close(index_of_left_parenthesis);
            }
        }

        // TODO finalize parsed function
        self.finalize_content()?;

        Ok(())
    }
}
