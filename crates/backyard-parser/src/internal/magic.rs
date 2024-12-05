use backyard_lexer::token::{ Token, TokenType };
use backyard_nodes::node::{ Location, MagicNode, Node, RangeLocation };

use crate::{
  error::ParserError,
  parser::{ LocationExtension, LocationHelper, LoopArgument, Parser },
  utils::{ match_pattern, Lookup, LookupResult, LookupResultWrapper },
};

#[derive(Debug, Clone)]
pub struct MagicParser;

impl MagicParser {
  pub fn from_token(id: &Token) -> Box<Node> {
    let start_loc = id.get_location().unwrap();
    let end_loc = start_loc.gen_end_loc(id.value.len());
    MagicNode::new(
      id.value.to_owned(),
      Some(RangeLocation {
        start: start_loc,
        end: end_loc,
      })
    )
  }

  pub fn test(tokens: &[Token], _: &mut LoopArgument) -> Option<Vec<LookupResult>> {
    match_pattern(tokens, &[Lookup::Equal(&[TokenType::Magic, TokenType::MagicMethod])])
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<LookupResult>,
    start_loc: Location,
    args: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [text] = matched.as_slice() {
      if let LookupResultWrapper::Equal(text) = &text.wrapper {
        return Ok(MagicNode::new(text.value.to_owned(), parser.gen_loc(start_loc)));
      };
    }
    Err(ParserError::internal("Magic", args))
  }
}
