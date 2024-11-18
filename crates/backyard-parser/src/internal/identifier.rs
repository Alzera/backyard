use backyard_lexer::token::{ Token, TokenType };
use backyard_nodes::node::{ Node, IdentifierNode };

use crate::{ parser::{ LoopArgument, Parser }, utils::some_or_default };

#[derive(Debug, Clone)]
pub struct IdentifierParser {}

impl IdentifierParser {
  pub fn new(name: String) -> Box<Node> {
    IdentifierNode::new(name)
  }

  pub fn from_matched(name: &Vec<Token>) -> Box<Node> {
    Self::new(some_or_default(name.get(0), String::from(""), |i| i.value.to_owned()))
  }
}

impl IdentifierParser {
  pub fn test(tokens: &Vec<Token>, _: &mut LoopArgument) -> Option<Vec<Vec<Token>>> {
    let token = tokens.get(0).unwrap();
    if [TokenType::Identifier].contains(&token.token_type) {
      return Some(vec![vec![token.to_owned()]]);
    }
    None
  }

  pub fn parse(
    _: &mut Parser,
    matched: Vec<Vec<Token>>,
    _: &mut LoopArgument
  ) -> Option<Box<Node>> {
    if let [identifier] = matched.as_slice() {
      return Some(IdentifierParser::from_matched(identifier));
    }
    None
  }
}
