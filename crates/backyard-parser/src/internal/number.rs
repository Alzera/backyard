use backyard_lexer::token::{ Token, TokenType };
use backyard_nodes::{ Location, Node, NumberNode };

use crate::{
  error::ParserError,
  parser::{ LoopArgument, Parser },
  utils::{ match_pattern, Lookup, LookupResult },
};

#[derive(Debug, Clone)]
pub struct NumberParser;

impl NumberParser {
  pub fn test<'arena, 'a>(
    parser: &mut Parser<'arena, 'a>,
    tokens: &[Token],
    _: &mut LoopArgument
  ) -> Option<std::vec::Vec<LookupResult<'arena>>> {
    match_pattern(
      parser,
      tokens,
      &[Lookup::Equal(&[TokenType::Number, TokenType::NumberHex, TokenType::NumberBinary])]
    )
  }

  pub fn parse<'arena, 'a, 'b>(
    parser: &mut Parser<'arena, 'a>,
    matched: std::vec::Vec<LookupResult>,
    start_loc: Location,
    _: &mut LoopArgument<'arena, 'b>
  ) -> Result<Node<'arena>, ParserError> {
    if let [number] = matched.as_slice() {
      return Ok(NumberNode::loc(number.as_equal()?.value.to_owned(), parser.gen_loc(start_loc)));
    }
    Err(ParserError::Internal)
  }
}
