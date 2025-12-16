use crate::{
    DataPath, DataPathSegment, DataPathValue, FuncDataPath, PipelineKernelErrorCode, PlainDataPath,
    ValueDataPath,
};
use std::ops::Deref;
use watchmen_model::{
    FactorType, StdErrCode, StdErrorCode, StdR, StringUtils, VariablePredefineFunctions,
};
use watchmen_runtime_model_kernel::{ArcFactor, TopicSchema};

struct ConsumingState<'a> {
    /// full path, usually for error report
    full_path: &'a str,
    /// all chars of full path
    all_chars: &'a Vec<char>,
    /// current char index of the char which read already
    char_index: usize,
    /// in-memory chars, not consumed yet
    in_memory_chars: String,
    // consumed segments
    segments: Vec<DataPathSegment>,
}

/// report error
impl ConsumingState<'_> {
    fn incorrect_char_at_index<R>(&self, reason: &str) -> StdR<R> {
        PipelineKernelErrorCode::IncorrectDataPath.msg(format!(
            "Incorrect data path[{}], caused by incorrect {} at index[{}].",
            self.full_path, reason, self.char_index
        ))
    }

    fn incorrect_dot<R>(&self) -> StdR<R> {
        self.incorrect_char_at_index("dot")
    }

    fn incorrect_comma<R>(&self) -> StdR<R> {
        self.incorrect_char_at_index("comma")
    }

    fn incorrect_left_parenthesis<R>(&self) -> StdR<R> {
        self.incorrect_char_at_index("left parenthesis")
    }

    fn incorrect_right_parenthesis<R>(&self) -> StdR<R> {
        self.incorrect_char_at_index("right parenthesis")
    }

    fn incorrect_left_brace<R>(&self) -> StdR<R> {
        self.incorrect_char_at_index("left brace")
    }

    fn incorrect_right_brace<R>(&self) -> StdR<R> {
        self.incorrect_char_at_index("right brace")
    }

    fn incorrect_ampersand<R>(&self) -> StdR<R> {
        self.incorrect_char_at_index("ampersand")
    }

    /// start is included, end is excluded
    fn incorrect_blank_segment<R>(
        &self,
        start_char_index: usize,
        end_char_index: usize,
    ) -> StdR<R> {
        PipelineKernelErrorCode::IncorrectDataPath.msg(format!(
            "Incorrect data path[{}], caused by blank segment at index[{}, {}].",
            self.full_path, start_char_index, end_char_index
        ))
    }

    fn unknown_error<R>(&self) -> StdR<R> {
        StdErrCode::Unknown.msg("Unknown error occurred during data path parsing.")
    }
}

/// utilities
impl ConsumingState<'_> {
    /// move char index to next
    fn move_char_index_to_next(&mut self) {
        self.char_index += 1;
    }

    /// get current char
    /// char index not change
    fn current_char(&self) -> Option<&char> {
        self.all_chars.get(self.char_index)
    }

    /// get char at given index.
    /// return none if index out of range
    fn char_at(&self, char_index: i64) -> Option<&char> {
        if char_index < 0 {
            None
        } else {
            self.all_chars.get(char_index as usize)
        }
    }

    /// check the in-memory chars is blank or not
    fn in_memory_chars_is_blank(&self) -> bool {
        self.in_memory_chars.is_blank()
    }

    /// check the in-memory chars is empty or not
    fn in_memory_chars_is_empty(&self) -> bool {
        self.in_memory_chars.is_empty()
    }

    /// check the in-memory chars is not empty or not
    fn in_memory_chars_is_not_empty(&self) -> bool {
        !self.in_memory_chars.is_empty()
    }

    /// get chars count of in-memory chars
    fn in_memory_chars_count(&self) -> usize {
        self.in_memory_chars.chars().count()
    }

    // clear in-memory chars
    fn clear_in_memory_chars(&mut self) {
        self.in_memory_chars.clear()
    }
}

/// consume chars
impl ConsumingState<'_> {
    /// append given char to in-memory chars
    /// and move char index to next
    fn consume_char_into_memory_and_keep_char_index(&mut self, char: char) {
        self.in_memory_chars.push(char);
    }

    /// append given char to in-memory chars
    /// and move char index to next
    fn consume_char_into_memory_and_move_char_index_to_next(&mut self, char: char) {
        self.in_memory_chars.push(char);
        self.move_char_index_to_next();
    }

    /// check the given char can be escaped or not
    /// if yes, append the escaped char to given str, move char index to index after the escaped char.
    /// otherwise append char '\' to given str, move char index to index after [\].
    fn consume_potential_escape_char(&mut self) {
        // current char index is point to the char "\", move to next
        self.move_char_index_to_next();

        if let Some(next_c) = self.current_char() {
            match next_c {
                '.' | ',' | '(' | ')' | '{' | '}' | '&' => {
                    self.consume_char_into_memory_and_move_char_index_to_next(*next_c);
                }
                _ => {
                    self.consume_char_into_memory_and_keep_char_index('\\');
                }
            }
        } else {
            self.consume_char_into_memory_and_keep_char_index('\\');
        }
    }
}

/// segments operations
impl ConsumingState<'_> {
    /// get last segment if it is a concat function
    /// raise error by given [not_found] when,
    /// - last segment is not a concat function,
    /// - segments is empty
    fn pop_last_concat_function(&mut self) -> Option<FuncDataPath> {
        if let Some(last_segment) = self.segments.last() {
            match last_segment {
                DataPathSegment::Func(data_path) => match data_path.func {
                    VariablePredefineFunctions::Concat => {
                        if let Some(segment) = self.segments.pop() {
                            match segment {
                                DataPathSegment::Func(data_path) => Some(data_path),
                                // never happen, checked already
                                _ => None,
                            }
                        } else {
                            // never happen, checked already
                            None
                        }
                    }
                    _ => None,
                },
                // last segment cannot be a plain path,
                // never happen since char before is "}", never be a plain path
                _ => None,
            }
        } else {
            // segments is empty
            None
        }
    }

    fn append_segment(&mut self, segment: DataPathSegment) {
        self.segments.push(segment);
    }
}

/// consume plain path
impl ConsumingState<'_> {
    /// create a plain data path, append to segments. and clear current chars.
    /// blank path is not allowed.
    fn consume_in_memory_chars_as_plain_path(&mut self, move_char_index_to_next: bool) -> StdR<()> {
        if self.in_memory_chars_is_blank() {
            return self.incorrect_blank_segment(
                self.char_index - self.in_memory_chars_count(),
                self.char_index,
            );
        }
        self.segments.push(DataPathSegment::Plain(PlainDataPath {
            path: self.in_memory_chars.clone(),
            is_vec: None,
        }));

        self.clear_in_memory_chars();
        if move_char_index_to_next {
            self.move_char_index_to_next()
        }
        Ok(())
    }
}

/// consume function
impl ConsumingState<'_> {
    /// try to consume function path, and return the char index after the function path.
    /// - if function has no param and no parenthesis, it stops at end of full path or any char not [a-zA-Z0-9_],
    ///   then return
    /// > any chars before function is not allowed.
    fn consume_function(&mut self) -> StdR<()> {
        if self.in_memory_chars_is_not_empty() {
            return self.incorrect_ampersand();
        }

        self.consume_char_into_memory_and_move_char_index_to_next('&');
        // TODO

        Ok(())
    }
}

/// for concat function of literal syntax
impl ConsumingState<'_> {
    /// get index of char before on-flying chars, when a literal concat function detected.
    /// current char is [{].
    ///
    /// if there is chars in in-memory chars, before it,
    /// otherwise before current char index
    fn get_index_of_char_before_literal_concat_function(&self) -> i64 {
        if self.in_memory_chars_is_not_empty() {
            // get char before in-memory chars
            (self.char_index - self.in_memory_chars_count() - 1) as i64
        } else {
            // get char before "{"
            (self.char_index - 1) as i64
        }
    }

    /// if char before in-memory chars (if exists) or [{] is
    /// - one of [.,(] or start of full path: should create,
    /// - [}], not create,
    /// - otherwise raise error
    fn should_create_concat_function(&self, index_of_char_before: i64) -> StdR<bool> {
        let char_before: Option<&char> = self.char_at(index_of_char_before);
        if let Some(char_before) = char_before {
            match char_before {
                '.' | ',' | '(' => Ok(true),
                '}' => Ok(false),
                _ => self.incorrect_left_brace()?,
            }
        } else {
            // no char before, it is the first part
            Ok(true)
        }
    }

    /// - if [should_create_concat_function] is true, create a concat function and return.
    ///   [path] and [params] of returned function is empty, since consuming not start yet.
    /// - or pop last element from segments, and return.
    ///   note if the last element is not a concat function, or segments is empty, raise error.
    fn get_or_create_concat_function(
        &mut self,
        should_create_concat_function: bool,
    ) -> StdR<FuncDataPath> {
        if should_create_concat_function {
            Ok(FuncDataPath {
                // leave the path empty, will set it later
                path: String::new(),
                func: VariablePredefineFunctions::Concat,
                params: Some(vec![]),
            })
        } else if let Some(data_path) = self.pop_last_concat_function() {
            // no need to create, and there is no segment in state, something wrong here!
            Ok(data_path)
        } else {
            self.incorrect_left_brace()
        }
    }

    /// left brace starts a sub path, it must be part of literal concat function.
    /// considering the following scenarios, for all following cases, [{ccc}] is sub path waiting for consuming:
    /// - not first part of concat function,
    ///   - [aaa{ccc}]: [aaa] already consumed, last segment in state is a plain path,
    ///   - [aaa{bbb}{ccc}]: [aaa{bbb}] already consumed, last segment in state is concat function path,
    /// - is first part of concat function,
    ///   - [aaa.{ccc}]: [aaa.] already consumed, last segment in state is a plain path,
    ///   - [aaa{bbb}.{ccc}]: [aaa{bbb}.] already consumed, last segment in state is concat function path,
    ///   - [&now.{ccc}]: [&now.] already consumed, last segment in state is a function path, but not concat,
    ///   - [&len({ccc})]: [&len(] already consumed, segment is empty,
    ///   - [&yearDiff(abc,{ccc}]: [[&yearDiff(abc, ] already consumed, segment is empty,
    /// and current char index is at the [{] of [{ccc}].
    ///
    /// so, need to check followings,
    /// - there are chars in in-memory chars: not first part, then check the char before in-memory chars,
    ///   - is one of [.,(] or start of full path:
    ///     - create concat function,
    ///     - let in-memory chars to be a string parameter part, is the first param of concat,
    ///   - is [}]: then check the last segment,
    ///     - is a concat function, use it,
    ///     - is not a concat function, raise error (logically never happens)
    ///   - is not one of [.,(}], raise error
    /// - the char before [{] is one of [.,(] or start of full path: is first part,
    ///   - create concat function,
    /// - the char before [{] is [}]: not first part, then check the last segment,
    ///   - is a concat function, use it,
    ///   - is not a concat function, raise error (logically never happens)
    /// - the char before [{] not one of [.,(}], raise error
    fn consume_literal_concat_function(&mut self) -> StdR<()> {
        let index_of_char_before = self.get_index_of_char_before_literal_concat_function();
        let should_create_concat_function =
            self.should_create_concat_function(index_of_char_before)?;
        let mut concat = self.get_or_create_concat_function(should_create_concat_function)?;
        // move all segments from existing concat function,
        let params = concat.params.take();
        let mut params = params.unwrap_or(vec![]);

        if self.in_memory_chars_is_not_empty() {
            // create a value path
            params.push(DataPathSegment::Value(ValueDataPath {
                path: self.in_memory_chars.clone(),
                value: DataPathValue::Str(self.in_memory_chars.clone()),
            }));
            self.clear_in_memory_chars();
        }

        // now create a state to continue consuming
        let mut sub_state = ConsumingState {
            full_path: self.full_path,
            all_chars: self.all_chars,
            char_index: self.char_index + 1,
            in_memory_chars: String::new(),
            segments: params,
        };
        sub_state.consume_sub_path()?;
        // reset the concat function path
        concat.path =
            self.full_path[(index_of_char_before + 1) as usize..sub_state.char_index].to_string();
        // copy params to concat function
        concat.params = Some(sub_state.segments);
        // copy char index to current state
        self.char_index = sub_state.char_index;
        // append the concat function to segments
        self.append_segment(DataPathSegment::Func(concat));

        Ok(())
    }
}

/// consume path
impl ConsumingState<'_> {
    /// path can contain multiple segments
    /// each segment can be,
    /// - starts with [&]: function,
    /// - not starts with [&], not contains [{}], plain,
    /// - not starts with [&], and contains [{}], function concat.
    /// and
    /// - cannot start with [().,],
    /// - cannot end with [.,],
    /// - before first [.], cannot be blank,
    /// - after last [.], cannot be blank,
    /// - for the literal concat function, functions that are not wrapped in `{}` are not allowed to appear.
    ///   e.g. [a.&len {b.len}] is not allowed.
    fn consume_path(&mut self) -> StdR<()> {
        loop {
            if let Some(char) = self.current_char() {
                match char {
                    // start of function, no content before function
                    '&' => self.consume_function()?,
                    '(' => self.incorrect_left_parenthesis()?,
                    ')' => self.incorrect_right_parenthesis()?,
                    // start of sub path
                    '{' => self.consume_literal_concat_function()?,
                    '}' => self.incorrect_right_brace()?,
                    // segment end
                    '.' => self.consume_in_memory_chars_as_plain_path(true)?,
                    ',' => self.incorrect_comma()?,
                    // potential escape char, check next char
                    '\\' => self.consume_potential_escape_char(),
                    // normal char, append to current chars
                    _ => self.consume_char_into_memory_and_move_char_index_to_next(*char),
                };
            } else {
                // reach the end of chars,
                // consume the chars in-memory as plain path
                self.consume_in_memory_chars_as_plain_path(false)?;
                break;
            }
        }

        Ok(())
    }

    fn consume_sub_path(&mut self) -> StdR<()> {
        // TODO
        Ok(())
    }
}

impl DataPath {
    /// factor name has no dot escape, no function
    pub fn from_factor(factor: &ArcFactor, topic_schema: &TopicSchema) -> StdR<DataPath> {
        let mut segments = vec![];
        let segment_paths: Vec<&str> = factor.name.split('.').collect();
        for (index, _) in segment_paths.iter().enumerate() {
            // each path is from start
            let path = segment_paths[0..(index + 1)].join(".");
            let factor = topic_schema.factor_by_name(&path);
            let is_vec = if let Some(factor) = factor {
                *factor.r#type.as_ref() == FactorType::Array
            } else {
                return PipelineKernelErrorCode::FactorNotFound.msg(format!(
                    "Factor[{}] not found in topic[{}].",
                    &path,
                    topic_schema.topic_id()
                ));
            };
            segments.push(DataPathSegment::Plain(PlainDataPath {
                path: segment_paths[index].to_string(),
                is_vec: Some(is_vec),
            }));
        }

        Ok(DataPath {
            path: factor.name.deref().clone(),
            segments,
        })
    }

    /// all kinds escapes, functions, variables
    /// - \. escapes dot,
    /// - \, escapes comma,
    /// - \( escapes left parenthesis,
    /// - \) escapes right parenthesis,
    /// - \{ escapes left brace,
    /// - \} escapes right brace,
    /// - \& escapes ampersand,
    /// - abc{ef} escapes path [ef],
    ///
    /// and fail fast
    pub fn from_str(path: &str) -> StdR<DataPath> {
        let mut state = ConsumingState {
            full_path: path,
            all_chars: &path.chars().collect(),
            char_index: 0,
            in_memory_chars: String::new(),
            segments: vec![],
        };
        state.consume_path()?;

        Ok(DataPath {
            path: path.to_string(),
            segments: state.segments,
        })
    }
}
