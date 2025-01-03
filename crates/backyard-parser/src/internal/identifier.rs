use backyard_lexer::token::{ Token, TokenType };
use backyard_nodes::{ IdentifierNode, Location, Node };

use crate::{
  error::ParserError,
  parser::{ LocationHelper, LoopArgument, Parser },
  utils::{ match_pattern, Lookup, LookupResult },
};

#[derive(Debug, Clone)]
pub struct IdentifierParser;

impl IdentifierParser {
  pub fn from_token<'arena>(id: &Token) -> Node<'arena> {
    let loc = id.get_range_location();
    IdentifierNode::loc(id.value.to_owned(), loc)
  }

  pub fn test<'arena, 'a>(
    parser: &mut Parser<'arena, 'a>,
    _: &mut LoopArgument
  ) -> Option<std::vec::Vec<LookupResult<'arena>>> {
    match_pattern(
      parser,
      &[
        Lookup::Equal(
          &[
            TokenType::UnqualifiedName,
            TokenType::QualifiedName,
            TokenType::RelativeName,
            TokenType::FullyQualifiedName,
            TokenType::Get,
            TokenType::Set,
          ]
        ),
      ]
    )
  }

  pub fn parse<'arena, 'a, 'b>(
    parser: &mut Parser<'arena, 'a>,
    matched: std::vec::Vec<LookupResult<'arena>>,
    _: Location,
    _: &mut LoopArgument<'arena, 'b>
  ) -> Result<Node<'arena>, ParserError> {
    if let [identifier] = matched.as_slice() {
      return Ok(Self::from_token(identifier.as_equal(parser)?));
    }
    Err(ParserError::Internal)
  }
}
