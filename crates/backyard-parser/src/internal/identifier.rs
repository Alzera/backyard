use backyard_lexer::token::{ Token, TokenType };
use backyard_nodes::node::{ Node, IdentifierNode };
use utils::guard;

use crate::{
  error::ParserError,
  parser::{ LoopArgument, Parser },
  utils::{ match_pattern, some_or_default, Lookup },
};

#[derive(Debug, Clone)]
pub struct IdentifierParser {}

impl IdentifierParser {
  pub fn new(name: String) -> Box<Node> {
    IdentifierNode::new(name)
  }

  pub fn from_matched(name: &Vec<Token>) -> Box<Node> {
    Self::new(some_or_default(name.get(0), String::from(""), |i| i.value.to_owned()))
  }
}

impl IdentifierParser {
  pub fn test(tokens: &Vec<Token>, _: &mut LoopArgument) -> Option<Vec<Vec<Token>>> {
    match_pattern(tokens, [Lookup::Equal(vec![TokenType::Identifier, TokenType::Name])].to_vec())
  }

  pub fn parse(
    _: &mut Parser,
    matched: Vec<Vec<Token>>,
    args: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [identifier] = matched.as_slice() {
      return Ok(
        IdentifierParser::new(
          guard!(identifier.get(0), {
            return Err(ParserError::internal("Identifier", args));
          }).value.to_owned()
        )
      );
    }
    Err(ParserError::internal("Identifier", args))
  }
}
