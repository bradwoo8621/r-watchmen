use crate::ParserInnerState;
use watchmen_model::StringUtils;

/// chars utilities
impl ParserInnerState<'_> {
    /// move char index to next
    pub fn move_char_index_to_next(&mut self) {
        self.char_index += 1;
    }

    /// get previous char
    /// char index not change
    pub fn previous_char(&self) -> Option<&char> {
        self.char_at(self.char_index as i64 - 1)
    }

    /// get current char
    /// char index not change
    pub fn current_char(&self) -> Option<&char> {
        self.all_chars.get(self.char_index)
    }

    /// get char at given index.
    /// return none if index out of range
    pub fn char_at(&self, char_index: i64) -> Option<&char> {
        if char_index < 0 {
            None
        } else {
            self.all_chars.get(char_index as usize)
        }
    }

    /// check the in-memory chars is blank or not
    pub fn in_memory_chars_is_blank(&self) -> bool {
        self.in_memory_chars.is_blank()
    }

    /// check the in-memory chars is empty or not
    pub fn in_memory_chars_is_empty(&self) -> bool {
        self.in_memory_chars.is_empty()
    }

    /// check the in-memory chars is not empty or not
    pub fn in_memory_chars_is_not_empty(&self) -> bool {
        !self.in_memory_chars.is_empty()
    }

    /// get chars count of in-memory chars
    pub fn in_memory_chars_count(&self) -> usize {
        self.in_memory_chars.chars().count()
    }

    // clear in-memory chars
    pub fn clear_in_memory_chars(&mut self) {
        self.in_memory_chars.clear()
    }
}
