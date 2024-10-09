use core::fmt;
use std::error::Error;

#[derive(Debug)]
pub enum ParseError {
  UnexpectedEOF,
  MismatchedClosingTag,
  InvalidTag,
  InvalidAttributeValue,
}

impl fmt::Display for ParseError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      ParseError::InvalidTag => write!(f, "Invalid HTML tag"),
      ParseError::UnexpectedEOF => write!(f, "Unexpected end of file"),
      ParseError::MismatchedClosingTag => write!(f, "Mismatched closing tag"),
      ParseError::InvalidAttributeValue => write!(f, "Invalid attribute value"),
    }
  }
}

impl Error for ParseError {}
