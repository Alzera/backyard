use backyard_lexer::token::{ Token, TokenType };
use backyard_nodes::node::{ IdentifierNode, Location, Node };

use crate::{
  error::ParserError,
  parser::{ LocationHelper, LoopArgument, Parser },
  utils::{ match_pattern, Lookup, LookupResult },
};

#[derive(Debug, Clone)]
pub struct IdentifierParser;

impl IdentifierParser {
  pub fn from_token(id: &Token) -> Box<Node> {
    let loc = id.get_range_location();
    IdentifierNode::loc(id.value.to_owned(), loc)
  }

  pub fn test(tokens: &[Token], _: &mut LoopArgument) -> Option<Vec<LookupResult>> {
    match_pattern(
      tokens,
      &[Lookup::Equal(&[TokenType::Identifier, TokenType::Name, TokenType::Get, TokenType::Set])]
    )
  }

  pub fn parse(
    _: &mut Parser,
    matched: Vec<LookupResult>,
    _: Location,
    _: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [identifier] = matched.as_slice() {
      return Ok(Self::from_token(identifier.as_equal()?));
    }
    Err(ParserError::Internal)
  }
}
