use crate::{
  lexer::token::{ Token, TokenType },
  parser::{
    node::Node,
    nodes::list::ListNode,
    parser::{ LoopArgument, Parser },
    utils::{ match_pattern, Lookup },
  },
};

#[derive(Debug, Clone)]
pub struct ListParser {}

impl ListParser {
  pub fn test(tokens: &Vec<Token>, _: &LoopArgument) -> Option<Vec<Vec<Token>>> {
    match_pattern(
      tokens,
      [
        Lookup::Equal(vec![TokenType::List]),
        Lookup::Equal(vec![TokenType::LeftParenthesis]),
      ].to_vec()
    )
  }

  pub fn parse(parser: &mut Parser, matched: Vec<Vec<Token>>, _: &LoopArgument) -> Option<Node> {
    if let [_, _] = matched.as_slice() {
      let values = parser.get_children(
        &mut LoopArgument::with_tokens("list", &[TokenType::Comma], &[TokenType::RightParenthesis])
      );
      return Some(ListNode::new(values));
    }
    None
  }
}
