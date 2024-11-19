use std::fmt::{ Display, Formatter };

use backyard_lexer::error::LexError;

use crate::parser::LoopArgument;

#[derive(Debug, Clone, PartialEq)]
pub enum ParserError {
  LexError(LexError),
  Internal {
    parser: String,
    args: String,
  },
  Failed(String),
}

impl ParserError {
  pub fn internal(parser: &str, args: &LoopArgument) -> Self {
    ParserError::Internal { parser: parser.to_string(), args: args.to_string() }
  }
}

impl Display for ParserError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      ParserError::LexError(err) => write!(f, "{}", err),
      ParserError::Internal { parser, args } => {
        write!(f, "Failed to parse: {:?}, {:?}", parser, args)
      }
      ParserError::Failed(reason) => write!(f, "Failed to parse: {}", reason),
    }
  }
}
