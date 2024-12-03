use backyard_lexer::token::{ Token, TokenType };
use backyard_nodes::node::{ Location, Node, MagicNode };

use crate::{
  error::ParserError,
  parser::{ LoopArgument, Parser },
  utils::{ match_pattern, Lookup, LookupResult, LookupResultWrapper },
};

#[derive(Debug, Clone)]
pub struct MagicParser;

impl MagicParser {
  pub fn test(tokens: &[Token], _: &mut LoopArgument) -> Option<Vec<LookupResult>> {
    match_pattern(tokens, &[Lookup::Equal(&[TokenType::Magic])])
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<LookupResult>,
    start_loc: Location,
    args: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [text] = matched.as_slice() {
      let text = if let LookupResultWrapper::Equal(text) = &text.wrapper {
        text.value.to_owned()
      } else {
        return Err(ParserError::internal("Magic", args));
      };
      return Ok(MagicNode::new(text, parser.gen_loc(start_loc)));
    }
    Err(ParserError::internal("Magic", args))
  }
}
