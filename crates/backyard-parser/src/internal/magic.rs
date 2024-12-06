use backyard_lexer::token::{ Token, TokenType };
use backyard_nodes::node::{ Location, MagicNode, Node };

use crate::{
  error::ParserError,
  parser::{ LocationHelper, LoopArgument, Parser },
  utils::{ match_pattern, Lookup, LookupResult, LookupResultWrapper },
};

#[derive(Debug, Clone)]
pub struct MagicParser;

impl MagicParser {
  pub fn from_token(id: &Token) -> Box<Node> {
    let loc = id.get_range_location();
    MagicNode::new(id.value.to_owned(), loc)
  }

  pub fn test(tokens: &[Token], _: &mut LoopArgument) -> Option<Vec<LookupResult>> {
    match_pattern(tokens, &[Lookup::Equal(&[TokenType::Magic, TokenType::MagicMethod])])
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<LookupResult>,
    start_loc: Location,
    _: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [text] = matched.as_slice() {
      if let LookupResultWrapper::Equal(text) = &text.wrapper {
        return Ok(MagicNode::new(text.value.to_owned(), parser.gen_loc(start_loc)));
      };
    }
    Err(ParserError::Internal)
  }
}
