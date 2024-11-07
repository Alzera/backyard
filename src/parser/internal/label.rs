use crate::{
  lexer::token::{ Token, TokenType },
  parser::{
    node::Node,
    nodes::label::LabelNode,
    parser::{ LoopArgument, Parser },
    utils::{ match_pattern, Lookup },
  },
};

use super::identifier::IdentifierParser;

#[derive(Debug, Clone)]
pub struct LabelParser {}

impl LabelParser {
  pub fn test(tokens: &Vec<Token>, _: &LoopArgument) -> Option<Vec<Vec<Token>>> {
    let grammar = [
      Lookup::Equal(vec![TokenType::Identifier]),
      Lookup::Equal(vec![TokenType::Colon]),
    ].to_vec();
    match_pattern(tokens, grammar)
  }

  pub fn parse(_: &mut Parser, matched: Vec<Vec<Token>>, _: &LoopArgument) -> Option<Node> {
    if let [name, _] = matched.as_slice() {
      return Some(LabelNode::new(IdentifierParser::from_matched(name)));
    }
    None
  }
}
