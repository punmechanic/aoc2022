#[cfg(test)]
mod tests {}
use std::{num::ParseIntError, string};

use clap::ValueEnum;

#[derive(Debug, Clone, ValueEnum)]
pub enum Part {
    Part1,
    Part2,
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    ParseIntError,
    IOError,
    Utf8Error,
}

impl From<ParseIntError> for Error {
    fn from(_: ParseIntError) -> Self {
        Self::ParseIntError
    }
}

impl From<std::io::Error> for Error {
    fn from(_: std::io::Error) -> Self {
        Self::IOError
    }
}

impl From<string::FromUtf8Error> for Error {
    fn from(_: string::FromUtf8Error) -> Self {
        Self::Utf8Error
    }
}
