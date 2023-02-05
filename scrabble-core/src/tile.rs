use std::fmt::Display;

use crate::letter::{Letter, ParseLetterError};
use thiserror::Error;

#[derive(Debug)]
pub struct Tile {
    pub letter: Option<Letter>,
    ty: TileType,
}

impl Tile {
    pub fn new(ty: TileType) -> Self {
        Self { letter: None, ty }
    }

    pub fn set(&mut self, char: char) -> Result<(), TileError> {
        if let Some(letter) = self.letter.as_mut() {
            *letter = Letter::try_from(char)?;

            return Ok(());
        };

        Err(TileError::Taken)
    }

    pub fn value(&self) -> Result<usize, TileError> {
        match &self.letter {
            Some(letter) => Ok(self.ty.get_letter_value(letter)),
            None => Err(TileError::Empty),
        }
    }
}

impl Default for Tile {
    fn default() -> Self {
        Self::new(TileType::default())
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let letter = match &self.letter {
            Some(letter) => letter.char,
            None => ' ',
        };

        write!(f, "[{letter}, {}]", self.ty)
    }
}

#[derive(Debug, Error)]
pub enum TileError {
    #[error("Tile has not been filled")]
    Empty,
    #[error("Tile is not a valid Scrabble letter")]
    Taken,
    #[error(transparent)]
    ParseLetter(#[from] ParseLetterError),
}

#[derive(Debug)]
pub enum TileType {
    Standard,
    DoubleLetter,
    TripleLetter,
    DoubleWord,
    TripleWord,
}

impl TileType {
    pub fn get_letter_value(&self, letter: &Letter) -> usize {
        match self {
            Self::DoubleLetter => letter.value * 2,
            Self::TripleLetter => letter.value * 3,
            _ => letter.value,
        }
    }
}

impl Default for TileType {
    fn default() -> Self {
        Self::Standard
    }
}

impl Display for TileType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            Self::Standard => "standard",
            Self::DoubleLetter => "double letter",
            Self::TripleLetter => "triple letter",
            Self::DoubleWord => "double letter",
            Self::TripleWord => "triple letter",
        };

        write!(f, "{message}")
    }
}

impl TryFrom<&str> for TileType {
    type Error = ParseTileTypeError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "s" => Ok(Self::Standard),
            "dl" => Ok(Self::DoubleLetter),
            "tl" => Ok(Self::TripleLetter),
            "dw" => Ok(Self::DoubleWord),
            "tw" => Ok(Self::TripleWord),
            _ => Err(ParseTileTypeError(Box::from(value))),
        }
    }
}

#[derive(Debug, Error)]
#[error("`{0}` is not a valid tile type")]
pub struct ParseTileTypeError(Box<str>);
