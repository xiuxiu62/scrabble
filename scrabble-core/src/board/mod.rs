mod default;

use crate::{
    array::Position,
    tile::{ParseTileTypeError, Tile, TileError, TileType},
};
use std::{fs, io, path::PathBuf};
use thiserror::Error;

#[derive(Debug)]
pub struct Board<const N: usize>([[Tile; N]; N]);

impl<const N: usize> Board<N> {
    pub fn new(tile_types: [[TileType; N]; N]) -> Self {
        Self(tile_types.map(|row| row.map(Tile::new)))
    }

    pub fn set(&mut self, position: Position, char: char) -> BoardResult<()> {
        self.tile_mut(&position)?
            .set(char)
            .map_err(|error| BoardError::Tile { position, error })
    }

    fn tile(&self, position: &Position) -> BoardResult<&Tile> {
        self.0
            .get(position.x())
            .and_then(|row| row.get(position.y()))
            .ok_or(BoardError::OutOfBounds(position.to_owned()))
    }

    fn tile_mut(&mut self, position: &Position) -> BoardResult<&mut Tile> {
        self.0
            .get_mut(position.x())
            .and_then(|row| row.get_mut(position.y()))
            .ok_or(BoardError::OutOfBounds(position.to_owned()))
    }
}

pub type BoardResult<T> = Result<T, BoardError>;

#[derive(Debug, Error)]
pub enum BoardError {
    #[error(transparent)]
    Parse(#[from] ParseBoardError),
    #[error("Tile error at index `{position}`: {error}")]
    Tile {
        position: Position,
        error: TileError,
    },
    #[error("The position `{0}` out of bounds")]
    OutOfBounds(Position),
}

impl<const N: usize> TryFrom<PathBuf> for Board<N> {
    type Error = ParseBoardError;

    fn try_from(path: PathBuf) -> Result<Self, Self::Error> {
        Self::try_from(fs::read_to_string(path)?)
    }
}

impl<const N: usize> TryFrom<String> for Board<N> {
    type Error = ParseBoardError;

    fn try_from(content: String) -> Result<Self, Self::Error> {
        let try_parse_line = |line: &str| -> Result<[TileType; N], ParseBoardError> {
            line.split_whitespace()
                .map(TileType::try_from)
                .collect::<Result<Vec<TileType>, ParseTileTypeError>>()?
                .try_into()
                .map_err(|_| ParseBoardError::InvalidSize { expected: N })
        };

        content
            .lines()
            .map(try_parse_line)
            .collect::<ParseBoardResult<Vec<[TileType; N]>>>()?
            .try_into()
            .map(Self::new)
            .map_err(|_| ParseBoardError::InvalidSize { expected: N })
    }
}

pub type ParseBoardResult<T> = Result<T, ParseBoardError>;

#[derive(Debug, Error)]
pub enum ParseBoardError {
    #[error(transparent)]
    InvalidType(#[from] ParseTileTypeError),
    #[error("Expected a board of size {expected}x{expected}")]
    InvalidSize { expected: usize },
    #[error(transparent)]
    InvalidPath(#[from] io::Error),
}

#[cfg(test)]
mod test {
    use super::Board;
    use std::path::PathBuf;

    #[test]
    fn board_deserialization_works() -> Result<(), Box<dyn std::error::Error>> {
        let board: Board<15> =
            Board::try_from(PathBuf::from("../../board-data/example-board.txt"))?;
        println!("{board:?}");

        Ok(())
    }
}
