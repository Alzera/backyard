use backyard_lexer::token::{ Token, TokenType };
use backyard_nodes::node::{ Node, EvalNode };

use crate::{
  error::ParserError,
  parser::{ LoopArgument, Parser },
  utils::{ match_pattern, Lookup },
};

#[derive(Debug, Clone)]
pub struct EvalParser;

impl EvalParser {
  pub fn test(tokens: &[Token], _: &mut LoopArgument) -> Option<Vec<Vec<Token>>> {
    match_pattern(
      tokens,
      [
        Lookup::Equal(vec![TokenType::Eval]),
        Lookup::Equal(vec![TokenType::LeftParenthesis]),
      ].to_vec()
    )
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<Vec<Token>>,
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
      return Ok(EvalNode::new(argument.unwrap()));
    }
    Err(ParserError::internal("Eval", args))
  }
}
