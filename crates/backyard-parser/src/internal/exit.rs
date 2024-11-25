use backyard_lexer::token::{ Token, TokenType };
use backyard_nodes::node::{ Node, ExitNode };

use crate::{
  error::ParserError,
  parser::{ LoopArgument, Parser },
  utils::{ match_pattern, Lookup },
};

#[derive(Debug, Clone)]
pub struct ExitParser {}

impl ExitParser {
  pub fn test(tokens: &Vec<Token>, _: &mut LoopArgument) -> Option<Vec<Vec<Token>>> {
    match_pattern(
      tokens,
      [
        Lookup::Equal(vec![TokenType::Exit, TokenType::Die]),
        Lookup::Optional(vec![TokenType::LeftParenthesis]),
      ].to_vec()
    )
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<Vec<Token>>,
    args: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [_, has_argument] = matched.as_slice() {
      let argument = if has_argument.len() > 0 {
        parser.get_statement(
          &mut LoopArgument::with_tokens("exit", &[], &[TokenType::RightParenthesis])
        )?
      } else {
        None
      };
      parser.position += 1;
      return Ok(ExitNode::new(argument));
    }
    Err(ParserError::internal("Exit", args))
  }
}
