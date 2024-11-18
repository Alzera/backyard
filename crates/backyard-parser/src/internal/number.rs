use backyard_lexer::token::{ Token, TokenType };
use backyard_nodes::{ node::{ Node, NumberNode } };

use crate::{ parser::{ LoopArgument, Parser }, utils::some_or_default };

#[derive(Debug, Clone)]
pub struct NumberParser {}

impl NumberParser {
  pub fn test(tokens: &Vec<Token>, _: &mut LoopArgument) -> Option<Vec<Vec<Token>>> {
    let token = tokens.get(0).unwrap();
    if [TokenType::Number, TokenType::NumberHex].contains(&token.token_type) {
      return Some(vec![vec![token.to_owned()]]);
    }
    None
  }

  pub fn parse(
    _: &mut Parser,
    matched: Vec<Vec<Token>>,
    _: &mut LoopArgument
  ) -> Option<Box<Node>> {
    if let [number] = matched.as_slice() {
      return Some(
        NumberNode::new(some_or_default(number.get(0), String::from("0"), |i| i.value.to_owned()))
      );
    }
    None
  }
}