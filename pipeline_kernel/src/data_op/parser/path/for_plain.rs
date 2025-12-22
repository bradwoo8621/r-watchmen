use crate::{DataPathSegment, PathParser, PlainDataPath};
use watchmen_model::StdR;

/// consume plain path
impl PathParser<'_> {
    /// create a plain data path, append to segments. and clear current chars.
    /// blank path is not allowed.
    pub fn consume_in_memory_chars_as_plain_path(
        &mut self,
        move_char_index_to_next: bool,
    ) -> StdR<()> {
        let inner = &mut self.inner;

        if inner.in_memory_chars_is_blank() {
            return self.incorrect_blank_segment();
        }

        self.segments.push(DataPathSegment::Plain(PlainDataPath {
            path: inner.clone_in_memory_chars(),
            is_vec: None,
        }));

        inner.clear_in_memory_chars();
        if move_char_index_to_next {
            inner.move_char_index_to_next()
        }
        Ok(())
    }
}
