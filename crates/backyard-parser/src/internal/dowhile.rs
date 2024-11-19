use backyard_lexer::token::{ Token, TokenType };
use backyard_nodes::node::{ Node, DoWhileNode };
use utils::guard;

use crate::{
  error::ParserError,
  parser::{ LoopArgument, Parser },
  utils::{ match_pattern, Lookup },
};

use super::block::BlockParser;

#[derive(Debug, Clone)]
pub struct DoWhileParser {}

impl DoWhileParser {
  pub fn test(tokens: &Vec<Token>, _: &mut LoopArgument) -> Option<Vec<Vec<Token>>> {
    match_pattern(tokens, [Lookup::Equal(vec![TokenType::Do])].to_vec())
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<Vec<Token>>,
    args: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [_] = matched.as_slice() {
      let body = BlockParser::new(parser)?;
      parser.position += 2;
      let condition = guard!(
        parser.get_statement(
          &mut LoopArgument::with_tokens("do_while", &[], &[TokenType::RightParenthesis])
        )?,
        {
          return Err(ParserError::internal("DoWhile", args));
        }
      );
      parser.position += 1;
      return Ok(DoWhileNode::new(condition, body));
    }
    Err(ParserError::internal("DoWhile", args))
  }
}
