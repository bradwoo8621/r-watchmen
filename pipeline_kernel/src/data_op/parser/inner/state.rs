pub struct ParserInnerState<'a> {
    /// full path, usually for error report
    pub full_path: &'a str,
    /// all chars of full path
    pub all_chars: &'a Vec<char>,
    /// current char index of the char which read already
    pub char_index: usize,
    /// in-memory chars, not consumed yet
    pub in_memory_chars: String,
}
