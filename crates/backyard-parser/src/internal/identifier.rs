use backyard_lexer::token::{ Token, TokenType };
use backyard_nodes::node::{ IdentifierNode, Location, Node, RangeLocation };

use crate::{
  error::ParserError,
  parser::{ LocationHelper, LoopArgument, Parser },
  utils::{ match_pattern, Lookup },
};

#[derive(Debug, Clone)]
pub struct IdentifierParser;

impl IdentifierParser {
  pub fn from_token(id: &Token) -> Box<Node> {
    let start_loc = id.get_location().unwrap();
    let id_len = id.value.len();
    let mut end_loc = start_loc.clone();
    end_loc.column += id_len;
    end_loc.offset += id_len;
    println!("id: {:?}, {:?}, {:?}", id, start_loc, end_loc);
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
    _: &mut Parser,
    matched: Vec<Vec<Token>>,
    _: Location,
    args: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [identifier] = matched.as_slice() {
      return Ok(Self::from_matched(&identifier));
    }
    Err(ParserError::internal("Identifier", args))
  }
}
