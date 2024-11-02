use crate::{
  guard,
  lexer::token::{ Token, TokenType },
  parser::{
    node::{ DoWhileNode, Node },
    parser::{ Internal, LoopArgument, Parser },
    utils::{ match_pattern, Lookup },
  },
};

use super::block::BlockParser;

#[derive(Debug, Clone)]
pub struct DoWhileParser {}

impl Internal for DoWhileParser {
  fn test(&self, tokens: &Vec<Token>, _: &LoopArgument) -> Option<Vec<Vec<Token>>> {
    match_pattern(tokens, [Lookup::Equal(vec![TokenType::Do])].to_vec())
  }

  fn parse(&self, parser: &mut Parser, matched: Vec<Vec<Token>>, _: &LoopArgument) -> Option<Node> {
    if let [_] = matched.as_slice() {
      let body = BlockParser::new(parser);
      parser.position += 2;
      let condition = guard!(
        parser.get_statement(
          &mut LoopArgument::with_tokens("do_while", &[], &[TokenType::RightParenthesis])
        )
      );
      parser.position += 1;
      return Some(
        Box::new(DoWhileNode {
          condition,
          body,
        })
      );
    }
    None
  }
}
