//! Error and result types.
use crate::bao::pv::PVLine;
use std::num::ParseIntError;
use std::sync::mpsc::SendError;

/// Search error type.
#[derive(Debug, thiserror::Error)]
pub enum SearchError {
    #[error("SendPVLineError: {0}")]
    SendPVLineError(#[from] SendError<PVLine>),
}

/// Search result type.
pub type SearchResult<T> = Result<T, SearchError>;

/// Move factory error type.
#[derive(Debug, thiserror::Error)]
pub enum MoveFactoryError {
    #[error("ParseError: {0}")]
    ParseError(#[from] ParseIntError),
    #[error("ParseInvalidIndex: {0}")]
    ParseInvalidIndex(i8),
    #[error("ParseInvalidDirection: {0}")]
    ParseInvalidDirection(String),
    #[error("ParseInvalidLength: {0}")]
    ParseInvalidLength(usize),
}

/// Move factory result type.
pub type MoveFactoryResult<T> = Result<T, MoveFactoryError>;

/// Game factory error type
#[derive(Debug, thiserror::Error)]
pub enum GameBuilderError {
    #[error("InvalidMove: {0}")]
    InvalidMove(#[from] MoveFactoryError),
    #[error("IllegalMove: {0}")]
    IllegalMove(String),
}

/// Game factory result type
pub type GameBuilderResult<T> = Result<T, GameBuilderError>;
