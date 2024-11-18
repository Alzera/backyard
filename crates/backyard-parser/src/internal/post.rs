use backyard_lexer::token::{ Token, TokenType };
use backyard_nodes::{ node::{ Node, PostNode } };

use crate::{ parser::{ LoopArgument, Parser }, utils::{ match_pattern, Lookup } };

#[derive(Debug, Clone)]
pub struct PostParser {}

impl PostParser {
  pub fn test(tokens: &Vec<Token>, _: &mut LoopArgument) -> Option<Vec<Vec<Token>>> {
    match_pattern(
      tokens,
      [Lookup::Equal(vec![TokenType::PostIncrement, TokenType::PostDecrement])].to_vec()
    )
  }

  pub fn parse(
    _: &mut Parser,
    matched: Vec<Vec<Token>>,
    args: &mut LoopArgument
  ) -> Option<Box<Node>> {
    if let [operator] = matched.as_slice() {
      if args.last_expr.is_none() {
        return None;
      }
      let operator = operator.get(0);
      if operator.is_none() {
        return None;
      }
      return Some(
        PostNode::new(args.last_expr.to_owned().unwrap(), operator.unwrap().value.to_owned())
      );
    }
    None
  }
}
