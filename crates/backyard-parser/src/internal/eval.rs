use backyard_lexer::token::{ Token, TokenType };
use backyard_nodes::node::{ Location, Node, EvalNode };

use crate::{
  error::ParserError,
  parser::{ LoopArgument, Parser },
  utils::{ match_pattern, Lookup, LookupResult },
};

#[derive(Debug, Clone)]
pub struct EvalParser;

impl EvalParser {
  pub fn test(tokens: &[Token], _: &mut LoopArgument) -> Option<Vec<LookupResult>> {
    match_pattern(
      tokens,
      &[Lookup::Equal(&[TokenType::Eval]), Lookup::Equal(&[TokenType::LeftParenthesis])]
    )
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<LookupResult>,
    start_loc: Location,
    args: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [_, _] = matched.as_slice() {
      let argument = parser.get_statement(
        &mut LoopArgument::with_tokens("eval", &[TokenType::RightParenthesis], &[])
      )?;
      parser.position += 1;
      if argument.is_none() {
        return Err(ParserError::internal("Eval", args));
      }
      return Ok(EvalNode::new(argument.unwrap(), parser.gen_loc(start_loc)));
    }
    Err(ParserError::internal("Eval", args))
  }
}
