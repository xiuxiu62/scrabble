mod default;
mod error;

use crate::{
    array::Position,
    letter::Letter,
    position,
    tile::{ParseTileTypeError, Tile, TileType},
};
pub use error::{
    Error as BoardError, ParseError as ParseBoardError, ParseResult as ParseBoardResult,
    Result as BoardResult,
};
use error::{Error, ParseError, ParseResult, Result};
use std::{fs, mem, path::PathBuf};

#[derive(Debug, PartialEq)]
pub struct Board<const N: usize>([[Tile; N]; N]);

impl<const N: usize> Board<N> {
    pub fn new(tile_types: [[TileType; N]; N]) -> Self {
        Self(tile_types.map(|row| row.map(Tile::new)))
    }

    pub fn set(&mut self, position: Position, char: char) -> Result<()> {
        self.tile_mut(&position)?
            .set(char)
            .map_err(|error| Error::Tile { position, error })
    }

    pub fn remove(&mut self, position: &Position) -> Result<Option<Letter>> {
        Ok(mem::take(&mut self.tile_mut(position)?.letter))
    }

    fn tile(&self, position: &Position) -> Result<&Tile> {
        self.0
            .get(position.x())
            .and_then(|row| row.get(position.y()))
            .ok_or(Error::OutOfBounds(position.to_owned()))
    }

    fn tile_mut(&mut self, position: &Position) -> Result<&mut Tile> {
        self.0
            .get_mut(position.x())
            .and_then(|row| row.get_mut(position.y()))
            .ok_or(Error::OutOfBounds(position.to_owned()))
    }

    pub fn iter(&self) -> Iter<'_, N> {
        self.into_iter()
    }
}

impl<const N: usize> TryFrom<PathBuf> for Board<N> {
    type Error = ParseError;

    fn try_from(path: PathBuf) -> ParseResult<Self> {
        Self::try_from(fs::read_to_string(path)?)
    }
}

impl<const N: usize> TryFrom<String> for Board<N> {
    type Error = ParseError;

    fn try_from(content: String) -> ParseResult<Self> {
        let try_parse_line = |line: &str| -> ParseResult<[TileType; N]> {
            line.split_whitespace()
                .map(TileType::try_from)
                .collect::<std::result::Result<Vec<TileType>, ParseTileTypeError>>()?
                .try_into()
                .map_err(|_| ParseError::InvalidSize { expected: N })
        };

        content
            .lines()
            .map(try_parse_line)
            .collect::<ParseResult<Vec<[TileType; N]>>>()?
            .try_into()
            .map(Self::new)
            .map_err(|_| ParseError::InvalidSize { expected: N })
    }
}

impl<const N: usize> IntoIterator for Board<N> {
    type Item = Tile;
    type IntoIter = IntoIter<N>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            board: self,
            position: position!(0, 0),
        }
    }
}

pub struct IntoIter<const N: usize> {
    board: Board<N>,
    position: Position,
}

impl<const N: usize> Iterator for IntoIter<N> {
    type Item = Tile;

    fn next(&mut self) -> Option<Self::Item> {
        let tile = self.board.tile_mut(&self.position).map(mem::take).ok();
        match self.position.x() {
            val if val == N - 1 => {
                self.position.set_x(0);
                self.position.set_y(self.position.y() + 1);
            }
            _ => self.position.set_x(self.position.x() + 1),
        };

        tile
    }
}

impl<'a, const N: usize> IntoIterator for &'a Board<N> {
    type Item = &'a Tile;
    type IntoIter = Iter<'a, N>;

    fn into_iter(self) -> Self::IntoIter {
        Iter {
            board: &self,
            position: position!(0, 0),
        }
    }
}

pub struct Iter<'a, const N: usize> {
    board: &'a Board<N>,
    position: Position,
}

impl<'a, const N: usize> Iterator for Iter<'a, N> {
    type Item = &'a Tile;

    fn next(&mut self) -> Option<Self::Item> {
        let tile = self.board.tile(&self.position).ok();
        match self.position.x() {
            val if val == N - 1 => {
                self.position.set_x(0);
                self.position.set_y(self.position.y() + 1);
            }
            _ => self.position.set_x(self.position.x() + 1),
        };

        tile
    }
}
