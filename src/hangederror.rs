use std::result;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum HangedError {
    #[error("Can't open file")]
    FileOpening,
    #[error("Can't flush")]
    Flushing,
    #[error("Can't parse line into word")]
    LineParsing,
    #[error("Can't read line")]
    LineReading,
}

pub type Result<T> = result::Result<T, HangedError>;
