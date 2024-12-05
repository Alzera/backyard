use backyard_lexer::token::{ Token, TokenType };
use backyard_nodes::node::{ IdentifierNode, Location, Node, RangeLocation };

use crate::{
  error::ParserError,
  parser::{ LocationExtension, LocationHelper, LoopArgument, Parser },
  utils::{ match_pattern, Lookup, LookupResult, LookupResultWrapper },
};

#[derive(Debug, Clone)]
pub struct IdentifierParser;

impl IdentifierParser {
  pub fn from_token(id: &Token) -> Box<Node> {
    let start_loc = id.get_location().unwrap();
    let end_loc = start_loc.gen_end_loc(id.value.len());
    IdentifierNode::new(
      id.value.to_owned(),
      Some(RangeLocation {
        start: start_loc,
        end: end_loc,
      })
    )
  }

  pub fn test(tokens: &[Token], _: &mut LoopArgument) -> Option<Vec<LookupResult>> {
    match_pattern(tokens, &[Lookup::Equal(&[TokenType::Identifier, TokenType::Name])])
  }

  pub fn parse(
    _: &mut Parser,
    matched: Vec<LookupResult>,
    _: Location,
    args: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [identifier] = matched.as_slice() {
      if let LookupResultWrapper::Equal(identifier) = &identifier.wrapper {
        return Ok(Self::from_token(identifier));
      }
    }
    Err(ParserError::internal("Identifier", args))
  }
}
