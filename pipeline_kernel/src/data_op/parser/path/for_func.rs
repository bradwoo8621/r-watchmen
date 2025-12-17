use crate::PathParser;
use watchmen_model::StdR;

/// consume function
impl PathParser<'_> {
    /// try to consume function path, and return the char index after the function path.
    /// - if function has no param and no parenthesis, it stops at end of full path or any char not [a-zA-Z0-9_],
    ///   then return
    /// > any chars before function is not allowed.
    pub fn consume_func_path(&mut self) -> StdR<()> {
        let inner = &mut self.inner;

        if inner.in_memory_chars_is_not_empty() {
            return self.inner.incorrect_ampersand();
        }

        inner.consume_char_into_memory_and_move_char_index_to_next('&');
        // TODO

        Ok(())
    }
}
