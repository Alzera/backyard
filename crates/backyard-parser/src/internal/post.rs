use backyard_lexer::token::{ Token, TokenType };
use backyard_nodes::node::{ Node, PostNode };

use crate::{
  error::ParserError,
  guard,
  parser::{ LoopArgument, Parser },
  utils::{ match_pattern, Lookup },
};

#[derive(Debug, Clone)]
pub struct PostParser;

impl PostParser {
  pub fn test(tokens: &[Token], _: &mut LoopArgument) -> Option<Vec<Vec<Token>>> {
    match_pattern(tokens, &[Lookup::Equal(&[TokenType::PostIncrement, TokenType::PostDecrement])])
  }

  pub fn parse(
    _: &mut Parser,
    matched: Vec<Vec<Token>>,
    args: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [operator] = matched.as_slice() {
      if args.last_expr.is_none() {
        return Err(ParserError::internal("Post", args));
      }
      let operator = guard!(operator.first(), {
        return Err(ParserError::internal("Post", args));
      });
      return Ok(PostNode::new(args.last_expr.to_owned().unwrap(), operator.value.to_owned()));
    }
    Err(ParserError::internal("Post", args))
  }
}
