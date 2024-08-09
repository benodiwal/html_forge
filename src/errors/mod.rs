use core::fmt;
use std::error::Error;

#[derive(Debug)]
pub enum ParseError {
    InvalidTag,
    UnexpectedEOF,
    MismatchedClosingTag
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::InvalidTag => write!(f, "Invalid HTML tag"),
            ParseError::UnexpectedEOF => write!(f, "Unexpected end of file"),
            ParseError::MismatchedClosingTag => write!(f, "Mismatched closing tag")
        }
    }
}

impl Error for ParseError {}
