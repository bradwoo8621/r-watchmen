use crate::{DataPathSegment, ParserInnerState};

pub struct PathParser<'a> {
    pub inner: ParserInnerState<'a>,
    pub segments: Vec<DataPathSegment>,
}

impl<'a> PathParser<'a> {
    pub fn by_path(full_path: &'a str, all_chars: &'a Vec<char>) -> Self {
        PathParser {
            inner: ParserInnerState {
                full_path,
                all_chars,
                char_index: 0,
                in_memory_chars: String::new(),
            },
            segments: vec![],
        }
    }
}
