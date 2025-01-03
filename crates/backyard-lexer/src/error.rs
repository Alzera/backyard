use std::fmt::{ Display, Formatter };

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
        write!(f, "Unrecognized character '{}' at line {}, column {}", token, line, column),
      LexError::Eof => write!(f, "End of file"),
    }
  }
}

pub type LexResult = Result<(), LexError>;
