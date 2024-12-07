use backyard_lexer::token::{ Token, TokenType };
use backyard_nodes::node::{ Location, Node, PostNode };

use crate::{
  error::ParserError,
  parser::{ LoopArgument, Parser },
  utils::{ match_pattern, Lookup, LookupResult },
};

#[derive(Debug, Clone)]
pub struct PostParser;

impl PostParser {
  pub fn test(tokens: &[Token], args: &mut LoopArgument) -> Option<Vec<LookupResult>> {
    args.last_expr.as_ref()?;
    match_pattern(tokens, &[Lookup::Equal(&[TokenType::PostIncrement, TokenType::PostDecrement])])
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<LookupResult>,
    start_loc: Location,
    args: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [operator] = matched.as_slice() {
      return Ok(
        PostNode::loc(
          args.last_expr.to_owned().unwrap(),
          operator.as_equal()?.value.to_owned(),
          parser.gen_loc(start_loc)
        )
      );
    }
    Err(ParserError::Internal)
  }
}
