use crate::{
  guard,
  lexer::token::{ Token, TokenType },
  parser::{
    node::Node,
    nodes::whiles::WhileNode,
    parser::{ LoopArgument, Parser },
    utils::{ match_pattern, Lookup },
  },
};

use super::block::BlockParser;

#[derive(Debug, Clone)]
pub struct WhileParser {}

impl WhileParser {
  pub fn test(tokens: &Vec<Token>, _: &LoopArgument) -> Option<Vec<Vec<Token>>> {
    match_pattern(
      tokens,
      [
        Lookup::Equal(vec![TokenType::While]),
        Lookup::Equal(vec![TokenType::LeftParenthesis]),
      ].to_vec()
    )
  }

  pub fn parse(parser: &mut Parser, matched: Vec<Vec<Token>>, _: &LoopArgument) -> Option<Node> {
    if let [_, _] = matched.as_slice() {
      let condition = guard!(
        parser.get_statement(
          &mut LoopArgument::with_tokens("while", &[], &[TokenType::RightParenthesis])
        )
      );
      parser.position += 1;
      let (is_short, body) = guard!(BlockParser::new_or_short(parser, &[TokenType::EndWhile]));
      return Some(WhileNode::new(condition, body, is_short));
    }
    None
  }
}
