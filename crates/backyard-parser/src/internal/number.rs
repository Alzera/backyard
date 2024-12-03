use backyard_lexer::token::{ Token, TokenType };
use backyard_nodes::node::{ Location, Node, NumberNode };

use crate::{
  error::ParserError,
  parser::{ LoopArgument, Parser },
  utils::{ match_pattern, Lookup, LookupResult, LookupResultWrapper },
};

#[derive(Debug, Clone)]
pub struct NumberParser;

impl NumberParser {
  pub fn test(tokens: &[Token], _: &mut LoopArgument) -> Option<Vec<LookupResult>> {
    match_pattern(tokens, &[Lookup::Equal(&[TokenType::Number, TokenType::NumberHex])])
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<LookupResult>,
    start_loc: Location,
    args: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [number] = matched.as_slice() {
      let number = if let LookupResultWrapper::Equal(number) = &number.wrapper {
        number.value.to_owned()
      } else {
        return Err(ParserError::internal("Number", args));
      };
      return Ok(NumberNode::new(number, parser.gen_loc(start_loc)));
    }
    Err(ParserError::internal("Number", args))
  }
}
