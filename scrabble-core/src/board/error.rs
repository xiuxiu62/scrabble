use crate::{
    array::Position,
    tile::{ParseTileTypeError, TileError},
};
use std::io;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Parse(#[from] ParseError),
    #[error("Tile error at index `{position}`: {error}")]
    Tile {
        position: Position,
        error: TileError,
    },
    #[error("The position `{0}` out of bounds")]
    OutOfBounds(Position),
}

pub type ParseResult<T> = std::result::Result<T, ParseError>;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error(transparent)]
    InvalidType(#[from] ParseTileTypeError),
    #[error("Expected a board of size {expected}x{expected}")]
    InvalidSize { expected: usize },
    #[error(transparent)]
    InvalidPath(#[from] io::Error),
}
