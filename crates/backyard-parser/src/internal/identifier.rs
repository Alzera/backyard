use backyard_lexer::token::{ Token, TokenType };
use backyard_nodes::node::{ IdentifierNode, Location, Node, RangeLocation };

use crate::{
  error::ParserError,
  guard,
  parser::{ LocationHelper, LoopArgument, Parser },
  utils::{ match_pattern, Lookup },
};

#[derive(Debug, Clone)]
pub struct IdentifierParser;

impl IdentifierParser {
  pub fn from_token(id: &Token) -> Box<Node> {
    let start_loc = id.get_location().unwrap();
    let mut end_loc = start_loc.clone();
    end_loc.column += id.value.len();
    IdentifierNode::new(
      id.value.to_owned(),
      Some(RangeLocation {
        start: start_loc,
        end: end_loc,
      })
    )
  }

  pub fn from_matched(name: &[Token]) -> Box<Node> {
    Self::from_token(name.first().unwrap())
  }
}

impl IdentifierParser {
  pub fn test(tokens: &[Token], _: &mut LoopArgument) -> Option<Vec<Vec<Token>>> {
    match_pattern(tokens, &[Lookup::Equal(&[TokenType::Identifier, TokenType::Name])])
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<Vec<Token>>,
    start_loc: Location,
    args: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [identifier] = matched.as_slice() {
      return Ok(
        IdentifierNode::new(
          guard!(identifier.first(), {
            return Err(ParserError::internal("Identifier", args));
          }).value.to_owned(),
          parser.gen_loc(start_loc)
        )
      );
    }
    Err(ParserError::internal("Identifier", args))
  }
}
