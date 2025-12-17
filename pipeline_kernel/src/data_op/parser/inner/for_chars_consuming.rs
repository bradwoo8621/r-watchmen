use crate::ParserInnerState;

/// consume chars
impl ParserInnerState<'_> {
    /// append given char to in-memory chars
    /// and move char index to next
    pub fn consume_char_into_memory_and_keep_char_index(&mut self, char: char) {
        self.in_memory_chars.push(char);
    }

    /// append given char to in-memory chars
    /// and move char index to next
    pub fn consume_char_into_memory_and_move_char_index_to_next(&mut self, char: char) {
        self.in_memory_chars.push(char);
        self.move_char_index_to_next();
    }

    /// check the given char can be escaped or not
    /// if yes, append the escaped char to given str, move char index to index after the escaped char.
    /// otherwise append char '\' to given str, move char index to index after [\].
    pub fn consume_potential_escape_char(&mut self) {
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
