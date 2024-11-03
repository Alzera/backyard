use crate::{
  lexer::token::{ Token, TokenType },
  parser::{
    node::Node,
    nodes::goto::GotoNode,
    parser::{ Internal, LoopArgument, Parser },
    utils::{ match_pattern, Lookup },
  },
};

use super::identifier::IdentifierParser;

#[derive(Debug, Clone)]
pub struct GotoParser {}

impl Internal for GotoParser {
  fn test(&self, tokens: &Vec<Token>, _: &LoopArgument) -> Option<Vec<Vec<Token>>> {
    let grammar = [
      Lookup::Equal(vec![TokenType::Goto]),
      Lookup::Equal(vec![TokenType::Identifier]),
    ].to_vec();
    match_pattern(tokens, grammar)
  }

  fn parse(&self, _: &mut Parser, matched: Vec<Vec<Token>>, _: &LoopArgument) -> Option<Node> {
    if let [_, identifier] = matched.as_slice() {
      return Some(GotoNode::new(IdentifierParser::from_matched(identifier)));
    }
    None
  }
}
