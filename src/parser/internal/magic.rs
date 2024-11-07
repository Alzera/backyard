use crate::{
  lexer::token::{ Token, TokenType },
  parser::{
    node::Node,
    nodes::magic::MagicNode,
    parser::{ LoopArgument, Parser },
    utils::{ match_pattern, some_or_default, Lookup },
  },
};

#[derive(Debug, Clone)]
pub struct MagicParser {}

impl MagicParser {
  pub fn test(tokens: &Vec<Token>, _: &LoopArgument) -> Option<Vec<Vec<Token>>> {
    match_pattern(tokens, [Lookup::Equal(vec![TokenType::Magic])].to_vec())
  }

  pub fn parse(_: &mut Parser, matched: Vec<Vec<Token>>, _: &LoopArgument) -> Option<Node> {
    if let [number] = matched.as_slice() {
      return Some(
        MagicNode::new(some_or_default(number.get(0), String::from("0"), |i| i.value.to_owned()))
      );
    }
    None
  }
}
