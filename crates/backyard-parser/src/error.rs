use std::fmt::{ Display, Formatter };

use backyard_lexer::error::LexError;
use backyard_nodes::node::Node;

#[derive(Debug, Clone, PartialEq)]
pub enum ParserError {
  LexError(LexError),
}

impl Display for ParserError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      ParserError::LexError(err) => write!(f, "{}", err),
    }
  }
}

pub type ParserResult = Result<Vec<Box<Node>>, ParserError>;
