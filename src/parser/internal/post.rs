use crate::{
  lexer::token::{ Token, TokenType },
  parser::{
    node::{ Node, PostNode },
    parser::{ Internal, LoopArgument, Parser },
    utils::{ match_pattern, Lookup },
  },
};

#[derive(Debug, Clone)]
pub struct PostParser {}

impl Internal for PostParser {
  fn test(&self, tokens: &Vec<Token>, _: &LoopArgument) -> Option<Vec<Vec<Token>>> {
    match_pattern(
      tokens,
      [Lookup::Equal(vec![TokenType::PostIncrement, TokenType::PostDecrement])].to_vec()
    )
  }

  fn parse(&self, _: &mut Parser, matched: Vec<Vec<Token>>, args: &LoopArgument) -> Option<Node> {
    if let [operator] = matched.as_slice() {
      if args.last_expr.is_none() {
        return None;
      }
      let operator = operator.get(0);
      if operator.is_none() {
        return None;
      }
      return Some(
        Box::new(PostNode {
          variable: args.last_expr.to_owned().unwrap(),
          operator: operator.unwrap().value.to_owned(),
        })
      );
    }
    None
  }
}
