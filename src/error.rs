use std::{
    error,
    fmt::{self, Display, Formatter},
};

use crate::MAGIC_HEADER_BYTES;

#[derive(Debug)]
pub enum Error {
    InvalidMagicHeaderString(String),
    InvalidPageSize(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::InvalidMagicHeaderString(v) => write!(
                f,
                "expected '{}', found {:?}",
                String::from_utf8_lossy(&MAGIC_HEADER_BYTES),
                v,
            ),
            Self::InvalidPageSize(msg) => write!(f, ""),
        }
    }
}

impl error::Error for Error {}
