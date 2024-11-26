use backyard_lexer::token::{ Token, TokenType };
use backyard_nodes::node::{ Node, ListNode };

use crate::{
  error::ParserError,
  parser::{ LoopArgument, Parser },
  utils::{ match_pattern, Lookup },
};

#[derive(Debug, Clone)]
pub struct ListParser;

impl ListParser {
  pub fn test(tokens: &[Token], _: &mut LoopArgument) -> Option<Vec<Vec<Token>>> {
    match_pattern(
      tokens,
      &[Lookup::Equal(&[TokenType::List]), Lookup::Equal(&[TokenType::LeftParenthesis])]
    )
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<Vec<Token>>,
    args: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [_, _] = matched.as_slice() {
      let values = parser.get_children(
        &mut LoopArgument::with_tokens("list", &[TokenType::Comma], &[TokenType::RightParenthesis])
      )?;
      return Ok(ListNode::new(values));
    }
    Err(ParserError::internal("List", args))
  }
}
