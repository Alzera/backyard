use std::fmt::{ Display, Formatter };

use crate::token::Token;

#[derive(Debug, Clone, PartialEq)]
pub enum LexError {
  Unrecognized(String),
  Eof,
}

impl Display for LexError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      LexError::Unrecognized(token) => write!(f, "unrecognized token: {}", token),
      LexError::Eof => write!(f, "no more tokens"),
    }
  }
}

pub type LexResult = Result<Vec<Token>, LexError>;
