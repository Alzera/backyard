use backyard_lexer::token::Token;
use backyard_nodes::node::{ Location, Node };

use crate::{
  error::ParserError,
  parser::{ LoopArgument, Parser },
  utils::{ match_pattern, Lookup, LookupResult, LookupResultWrapper },
};

#[derive(Debug, Clone)]
pub struct TypesParser;

impl TypesParser {
  #[allow(unused_assignments)]
  pub fn test(tokens: &[Token], _: &mut LoopArgument) -> Option<Vec<LookupResult>> {
    if let Some(m) = match_pattern(tokens, &[Lookup::OptionalType]) {
      if let Some(types) = m.first() {
        if !types.is_empty() {
          return Some(m);
        }
      }
    }
    None
  }

  pub fn parse(
    _: &mut Parser,
    matched: Vec<LookupResult>,
    _: Location,
    _: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [types] = matched.as_slice() {
      if let LookupResultWrapper::OptionalType(Some(types)) = &types.wrapper {
        return Ok(types.to_owned());
      }
    }
    Err(ParserError::Internal)
  }
}
