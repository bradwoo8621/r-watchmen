use crate::{AnyFuncParser, FuncDataPathParam, LiteralConcatFuncParser};

impl AnyFuncParser for LiteralConcatFuncParser {
    /// append given parameter to the last of params
    fn append_param(&mut self, param: FuncDataPathParam) {
        self.params.push(param);
    }

    fn start_char_index(&self) -> usize {
        self.inner.current_char_index()
    }

    fn move_char_index_to(&mut self, char_index: usize) {
        self.inner.move_char_index_to(char_index);
    }
}
