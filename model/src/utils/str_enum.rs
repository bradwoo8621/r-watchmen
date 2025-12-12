use std::collections::HashMap;

#[derive(Clone)]
pub enum CharsMatch {
    Next(HashMap<char, CharsMatch>),
    None,
}

impl CharsMatch {
    pub fn of(&self, char: &char) -> &CharsMatch {
        match self {
            CharsMatch::Next(map) => map.get(char).unwrap_or(&CharsMatch::None),
            CharsMatch::None => &CharsMatch::None,
        }
    }

    pub fn matches(&self, chars: &Vec<char>) -> bool {
        let mut chars_match = self;
        for char in chars {
            chars_match = chars_match.of(char);
            match chars_match {
                CharsMatch::None => return false,
                _ => {}
            }
        }
        match chars_match {
            CharsMatch::None => false,
            _ => true,
        }
    }
}
