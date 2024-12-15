use backyard_lexer::token::TokenType;
use backyard_nodes::{ Location, Node, PostNode, utils::IntoBoxedNode };

use crate::{
  error::ParserError,
  parser::{ LoopArgument, Parser },
  utils::{ match_pattern, Lookup, LookupResult },
};

#[derive(Debug, Clone)]
pub struct PostParser;

impl PostParser {
  pub fn test<'arena, 'a>(
    parser: &mut Parser<'arena, 'a>,
    args: &mut LoopArgument
  ) -> Option<std::vec::Vec<LookupResult<'arena>>> {
    args.last_expr.as_ref()?;
    match_pattern(parser, &[Lookup::Equal(&[TokenType::PostIncrement, TokenType::PostDecrement])])
  }

  pub fn parse<'arena, 'a, 'b>(
    parser: &mut Parser<'arena, 'a>,
    matched: std::vec::Vec<LookupResult<'arena>>,
    start_loc: Location,
    args: &mut LoopArgument<'arena, 'b>
  ) -> Result<Node<'arena>, ParserError> {
    if let [operator] = matched.as_slice() {
      return Ok(
        PostNode::loc(
          args.last_expr.take().unwrap().into_boxed(parser.arena),
          operator.as_equal(parser)?.value.to_owned(),
          parser.gen_loc(start_loc)
        )
      );
    }
    Err(ParserError::Internal)
  }
}
