use lazy_static::lazy_static;
use std::collections::HashMap;
use thiserror::Error;

macro_rules! hashmap {
($($key:expr => $value:expr),*) => {{
    let mut map = HashMap::new();
    $(map.insert($key, $value);)*

    map
}};
}

lazy_static! {
    pub static ref CHAR_VALUE_LOOKUP_TABLE: HashMap<char, usize> = hashmap! {
        'a' => 1,
        'b' => 3,
        'c' => 3,
        'd' => 2,
        'e' => 1,
        'f' => 4,
        'g' => 2,
        'h' => 4,
        'i' => 1,
        'j' => 8,
        'k' => 5,
        'l' => 1,
        'm' => 3,
        'n' => 1,
        'o' => 1,
        'p' => 3,
        'q' => 10,
        'r' => 1,
        's' => 1,
        't' => 1,
        'u' => 1,
        'v' => 4,
        'w' => 4,
        'x' => 8,
        'y' => 4,
        'z' => 10
    };
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Letter {
    pub char: char,
    pub value: usize,
}

impl TryFrom<char> for Letter {
    type Error = ParseLetterError;

    fn try_from(char: char) -> Result<Self, Self::Error> {
        match CHAR_VALUE_LOOKUP_TABLE.get(&char) {
            Some(value) => Ok(Self {
                char,
                value: *value,
            }),
            None => Err(ParseLetterError(char)),
        }
    }
}

#[derive(Debug, Error)]
#[error("`{0}` is not a valid Scrabble letter")]
pub struct ParseLetterError(char);
