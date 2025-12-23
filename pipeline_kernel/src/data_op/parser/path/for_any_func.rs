use crate::{
    DataPath, DataPathSegment, FuncDataPathParam, FuncParamValue, FuncParamValuePath, PathParser,
};

pub trait AnyFuncParser {
    fn append_param(&mut self, param: FuncDataPathParam);

    fn start_char_index(&self) -> usize;

    fn move_char_index_to(&mut self, char_index: usize);
}

impl PathParser {
    pub fn hand_over_to_func(self, any_func: &mut impl AnyFuncParser) {
        let mut segments = self.segments;
        if segments.is_empty() {
            // no segment, basically, it is a "{}", treated as an empty string
            any_func.append_param(FuncDataPathParam::Value(FuncParamValuePath {
                // TODO the start char index should minus 1 or not?
                path: self
                    .inner
                    .create_path_str_exclude_current(self.inner.current_char_index()),
                value: FuncParamValue::Str(String::from("")),
            }))
        } else if segments.len() > 1 {
            any_func.append_param(FuncDataPathParam::Path(DataPath {
                path: self
                    .inner
                    .create_path_str_exclude_current(any_func.start_char_index()),
                segments,
            }))
        } else {
            match segments.pop().unwrap() {
                DataPathSegment::Plain(plain_path) => {
                    any_func.append_param(FuncDataPathParam::Plain(plain_path))
                }
                DataPathSegment::Func(func_path) => {
                    any_func.append_param(FuncDataPathParam::Func(func_path))
                }
            }
        }
        // copy char index to current state
        any_func.move_char_index_to(self.inner.current_char_index());
    }
}
