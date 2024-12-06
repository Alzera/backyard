use std::fmt::{ Display, Formatter };

use backyard_lexer::{ error::LexError, token::Token };

#[derive(Debug, Clone, PartialEq)]
pub enum ParserError {
  LexError(LexError),
  Internal,
  Eof,
  UnexpectedToken(Token),
}

impl Display for ParserError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      ParserError::LexError(err) => write!(f, "{}", err),
      ParserError::Internal => { write!(f, "Internal parser error") }
      ParserError::Eof => { write!(f, "End of file") }
      ParserError::UnexpectedToken(token) => {
        write!(
          f,
          "Unexpected character '{}' at line {}, column {}",
          token.value,
          token.line,
          token.column
        )
      }
    }
  }
}
