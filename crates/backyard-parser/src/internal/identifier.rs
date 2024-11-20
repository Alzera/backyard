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
    match_pattern(
      tokens,
      [
        Lookup::Optional(vec![TokenType::BackSlash]),
        Lookup::Equal(vec![TokenType::Identifier]),
      ].to_vec()
    )
  }

  pub fn parse(
    _: &mut Parser,
    matched: Vec<Vec<Token>>,
    args: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [backlash, identifier] = matched.as_slice() {
      let id = guard!(identifier.get(0), {
        return Err(ParserError::internal("Identifier", args));
      });
      let name = if backlash.len() > 0 { format!("\\{}", id.value) } else { id.value.to_owned() };
      return Ok(IdentifierParser::new(name));
    }
    Err(ParserError::internal("Identifier", args))
  }
}
