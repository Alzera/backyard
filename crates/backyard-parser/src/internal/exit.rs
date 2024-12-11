use backyard_lexer::token::{ Token, TokenType };
use backyard_nodes::{ node::{ ExitNode, Location, Node }, utils::IntoBoxedOptionNode };

use crate::{
  error::ParserError,
  parser::{ LoopArgument, Parser },
  utils::{ match_pattern, Lookup, LookupResult },
};

#[derive(Debug, Clone)]
pub struct ExitParser;

impl ExitParser {
  pub fn test<'arena, 'a>(
    parser: &mut Parser<'arena, 'a>,
    tokens: &[Token],
    _: &mut LoopArgument
  ) -> Option<std::vec::Vec<LookupResult<'arena>>> {
    match_pattern(
      parser,
      tokens,
      &[
        Lookup::Equal(&[TokenType::Exit, TokenType::Die]),
        Lookup::Optional(&[TokenType::LeftParenthesis]),
      ]
    )
  }

  pub fn parse<'arena, 'a, 'b>(
    parser: &mut Parser<'arena, 'a>,
    matched: std::vec::Vec<LookupResult>,
    start_loc: Location,
    _: &mut LoopArgument<'arena, 'b>
  ) -> Result<Node<'arena>, ParserError> {
    if let [_, has_argument] = matched.as_slice() {
      let argument = if !has_argument.is_empty() {
        parser.get_statement(
          &mut LoopArgument::with_tokens(parser.arena, "exit", &[], &[TokenType::RightParenthesis])
        )?
      } else {
        None
      };
      parser.position += 1;
      return Ok(ExitNode::loc(argument.into_boxed(parser.arena), parser.gen_loc(start_loc)));
    }
    Err(ParserError::Internal)
  }
}
