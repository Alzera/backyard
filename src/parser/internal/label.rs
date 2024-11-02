use crate::{
  lexer::token::{ Token, TokenType },
  parser::{
    node::{ LabelNode, Node },
    parser::{ Internal, LoopArgument, Parser },
    utils::{ match_pattern, Lookup },
  },
};

use super::identifier::IdentifierParser;

#[derive(Debug, Clone)]
pub struct LabelParser {}

impl Internal for LabelParser {
  fn test(&self, tokens: &Vec<Token>, _: &LoopArgument) -> Option<Vec<Vec<Token>>> {
    let grammar = [
      Lookup::Equal(vec![TokenType::Identifier]),
      Lookup::Equal(vec![TokenType::Colon]),
    ].to_vec();
    match_pattern(tokens, grammar)
  }

  fn parse(&self, _: &mut Parser, matched: Vec<Vec<Token>>, _: &LoopArgument) -> Option<Node> {
    if let [name, _] = matched.as_slice() {
      return Some(
        Box::new(LabelNode {
          label: IdentifierParser::from_matched(name),
        })
      );
    }
    None
  }
}
