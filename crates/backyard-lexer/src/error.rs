use std::fmt::{ Display, Formatter };

use crate::token::Token;

#[derive(Debug, Clone, PartialEq)]
pub enum LexError {
  Unrecognized {
    token: String,
    line: usize,
    column: usize,
  },
  Eof,
}

impl Display for LexError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      LexError::Unrecognized { token, line, column } =>
        write!(f, "unrecognized token '{}' at line {}, column {}", token, line, column),
      LexError::Eof => write!(f, "no more tokens"),
    }
  }
}

pub type LexResult = Result<Vec<Token>, LexError>;
