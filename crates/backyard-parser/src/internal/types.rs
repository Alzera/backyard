use backyard_lexer::token::Token;
use backyard_nodes::node::{ Location, Node };

use crate::{
  error::ParserError,
  parser::{ LoopArgument, Parser },
  utils::{ match_pattern, Lookup, LookupResult },
};

#[derive(Debug, Clone)]
pub struct TypesParser;

impl TypesParser {
  #[allow(unused_assignments)]
  pub fn test<'arena, 'a>(
    parser: &mut Parser<'arena, 'a>,
    tokens: &[Token],
    _: &mut LoopArgument
  ) -> Option<std::vec::Vec<LookupResult<'arena>>> {
    if let Some(m) = match_pattern(parser, tokens, &[Lookup::OptionalType]) {
      if let Some(types) = m.first() {
        if !types.is_empty() {
          return Some(m);
        }
      }
    }
    None
  }

  pub fn parse<'arena, 'a, 'b>(
    parser: &mut Parser<'arena, 'a>,
    matched: std::vec::Vec<LookupResult>,
    _: Location,
    _: &mut LoopArgument<'arena, 'b>
  ) -> Result<Node<'arena>, ParserError> {
    if let [types] = matched.as_slice() {
      if let Some(types) = types.as_optional_type(&parser.arena) {
        return Ok(types);
      }
    }
    Err(ParserError::Internal)
  }
}
