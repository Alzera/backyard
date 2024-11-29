use backyard_lexer::token::{ Token, TokenType };
use backyard_nodes::node::{ Location, Node, MagicNode };

use crate::{
  error::ParserError,
  parser::{ LoopArgument, Parser },
  utils::{ match_pattern, Lookup },
};

#[derive(Debug, Clone)]
pub struct MagicParser;

impl MagicParser {
  pub fn test(tokens: &[Token], _: &mut LoopArgument) -> Option<Vec<Vec<Token>>> {
    match_pattern(tokens, &[Lookup::Equal(&[TokenType::Magic])])
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<Vec<Token>>,
    start_loc: Location,
    args: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [number] = matched.as_slice() {
      return Ok(
        MagicNode::new(
          number
            .first()
            .and_then(|i| Some(i.value.to_owned()))
            .or(Some("0".to_string()))
            .unwrap(),
          parser.gen_loc(start_loc)
        )
      );
    }
    Err(ParserError::internal("Magic", args))
  }
}
