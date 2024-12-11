use backyard_lexer::token::{ Token, TokenType };
use backyard_nodes::{ node::{ EvalNode, Location, Node }, utils::IntoBoxedNode };

use crate::{
  error::ParserError,
  parser::{ LoopArgument, Parser },
  utils::{ match_pattern, Lookup, LookupResult },
};

#[derive(Debug, Clone)]
pub struct EvalParser;

impl EvalParser {
  pub fn test<'arena, 'a>(
    parser: &mut Parser<'arena, 'a>,
    tokens: &[Token],
    _: &mut LoopArgument
  ) -> Option<std::vec::Vec<LookupResult<'arena>>> {
    match_pattern(
      parser,
      tokens,
      &[Lookup::Equal(&[TokenType::Eval]), Lookup::Equal(&[TokenType::LeftParenthesis])]
    )
  }

  pub fn parse<'arena, 'a, 'b>(
    parser: &mut Parser<'arena, 'a>,
    matched: std::vec::Vec<LookupResult>,
    start_loc: Location,
    _: &mut LoopArgument<'arena, 'b>
  ) -> Result<Node<'arena>, ParserError> {
    if let [_, _] = matched.as_slice() {
      if
        let Some(argument) = parser.get_statement(
          &mut LoopArgument::with_tokens(parser.arena, "eval", &[TokenType::RightParenthesis], &[])
        )?
      {
        parser.position += 1;
        return Ok(EvalNode::loc(argument.into_boxed(&parser.arena), parser.gen_loc(start_loc)));
      }
    }
    Err(ParserError::Internal)
  }
}
