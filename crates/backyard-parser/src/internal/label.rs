use backyard_lexer::token::{ Token, TokenType };
use backyard_nodes::node::{ Node, LabelNode };

use crate::{
  error::ParserError,
  parser::{ LoopArgument, Parser },
  utils::{ match_pattern, Lookup },
};

use super::identifier::IdentifierParser;

#[derive(Debug, Clone)]
pub struct LabelParser {}

impl LabelParser {
  pub fn test(tokens: &Vec<Token>, _: &mut LoopArgument) -> Option<Vec<Vec<Token>>> {
    let grammar = [
      Lookup::Equal(vec![TokenType::Identifier]),
      Lookup::Equal(vec![TokenType::Colon]),
    ].to_vec();
    match_pattern(tokens, grammar)
  }

  pub fn parse(
    _: &mut Parser,
    matched: Vec<Vec<Token>>,
    args: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [name, _] = matched.as_slice() {
      return Ok(LabelNode::new(IdentifierParser::from_matched(name)));
    }
    Err(ParserError::internal("Label", args))
  }
}
