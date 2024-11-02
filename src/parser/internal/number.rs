use crate::{
  lexer::token::{ Token, TokenType },
  parser::{
    node::{ Node, NumberNode },
    parser::{ Internal, LoopArgument, Parser },
    utils::some_or_default,
  },
};

#[derive(Debug, Clone)]
pub struct NumberParser {}

impl Internal for NumberParser {
  fn test(&self, tokens: &Vec<Token>, _: &LoopArgument) -> Option<Vec<Vec<Token>>> {
    let token = tokens.get(0).unwrap();
    if [TokenType::Number, TokenType::NumberHex].contains(&token.token_type) {
      return Some(vec![vec![token.to_owned()]]);
    }
    None
  }

  fn parse(&self, _: &mut Parser, matched: Vec<Vec<Token>>, _: &LoopArgument) -> Option<Node> {
    if let [number] = matched.as_slice() {
      return Some(
        Box::new(NumberNode {
          value: some_or_default(number.get(0), String::from("0"), |i| i.value.to_owned()),
        })
      );
    }
    None
  }
}
