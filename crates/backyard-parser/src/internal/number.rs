use backyard_lexer::token::{ Token, TokenType };
use backyard_nodes::node::{ Location, Node, NumberNode };

use crate::{
  error::ParserError,
  parser::{ LoopArgument, Parser },
  utils::{ match_pattern, Lookup, LookupResult },
};

#[derive(Debug, Clone)]
pub struct NumberParser;

impl NumberParser {
  pub fn test(tokens: &[Token], _: &mut LoopArgument) -> Option<Vec<LookupResult>> {
    match_pattern(
      tokens,
      &[Lookup::Equal(&[TokenType::Number, TokenType::NumberHex, TokenType::NumberBinary])]
    )
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<LookupResult>,
    start_loc: Location,
    _: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [number] = matched.as_slice() {
      return Ok(NumberNode::loc(number.as_equal()?.value.to_owned(), parser.gen_loc(start_loc)));
    }
    Err(ParserError::Internal)
  }
}
