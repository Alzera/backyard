use crate::{
  lexer::token::{ Token, TokenType },
  parser::{
    node::{ IdentifierNode, Node },
    parser::{ Internal, LoopArgument, Parser },
    utils::some_or_default,
  },
};

#[derive(Debug, Clone)]
pub struct IdentifierParser {}

impl IdentifierParser {
  pub fn new(name: String) -> Node {
    Box::new(IdentifierNode { name })
  }

  pub fn from_matched(name: &Vec<Token>) -> Node {
    Self::new(some_or_default(name.get(0), String::from(""), |i| i.value.to_owned()))
  }
}

impl Internal for IdentifierParser {
  fn test(&self, tokens: &Vec<Token>, _: &LoopArgument) -> Option<Vec<Vec<Token>>> {
    let token = tokens.get(0).unwrap();
    if [TokenType::Identifier].contains(&token.token_type) {
      return Some(vec![vec![token.to_owned()]]);
    }
    None
  }

  fn parse(&self, _: &mut Parser, matched: Vec<Vec<Token>>, _: &LoopArgument) -> Option<Node> {
    if let [identifier] = matched.as_slice() {
      return Some(IdentifierParser::from_matched(identifier));
    }
    None
  }
}
