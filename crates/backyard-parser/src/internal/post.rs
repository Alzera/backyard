use backyard_lexer::token::{ Token, TokenType };
use backyard_nodes::node::{ Location, Node, PostNode };

use crate::{
  error::ParserError,
  parser::{ LoopArgument, Parser },
  utils::{ match_pattern, Lookup, LookupResult, LookupResultWrapper },
};

#[derive(Debug, Clone)]
pub struct PostParser;

impl PostParser {
  pub fn test(tokens: &[Token], _: &mut LoopArgument) -> Option<Vec<LookupResult>> {
    match_pattern(tokens, &[Lookup::Equal(&[TokenType::PostIncrement, TokenType::PostDecrement])])
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<LookupResult>,
    start_loc: Location,
    args: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [operator] = matched.as_slice() {
      if args.last_expr.is_none() {
        return Err(ParserError::Internal);
      }
      let operator = if let LookupResultWrapper::Equal(operator) = &operator.wrapper {
        operator
      } else {
        return Err(ParserError::Internal);
      };
      return Ok(
        PostNode::new(
          args.last_expr.to_owned().unwrap(),
          operator.value.to_owned(),
          parser.gen_loc(start_loc)
        )
      );
    }
    Err(ParserError::Internal)
  }
}
