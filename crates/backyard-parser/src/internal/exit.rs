use backyard_lexer::token::{ Token, TokenType };
use backyard_nodes::node::{ Location, Node, ExitNode };

use crate::{
  error::ParserError,
  parser::{ LoopArgument, Parser },
  utils::{ match_pattern, Lookup, LookupResult },
};

#[derive(Debug, Clone)]
pub struct ExitParser;

impl ExitParser {
  pub fn test(tokens: &[Token], _: &mut LoopArgument) -> Option<Vec<LookupResult>> {
    match_pattern(
      tokens,
      &[
        Lookup::Equal(&[TokenType::Exit, TokenType::Die]),
        Lookup::Optional(&[TokenType::LeftParenthesis]),
      ]
    )
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<LookupResult>,
    start_loc: Location,
    _: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [_, has_argument] = matched.as_slice() {
      let argument = if !has_argument.is_empty() {
        parser.get_statement(
          &mut LoopArgument::with_tokens("exit", &[], &[TokenType::RightParenthesis])
        )?
      } else {
        None
      };
      parser.position += 1;
      return Ok(ExitNode::loc(argument, parser.gen_loc(start_loc)));
    }
    Err(ParserError::Internal)
  }
}
