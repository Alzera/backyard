use backyard_lexer::token::{ Token, TokenType };
use backyard_nodes::{ node::{ Node, DoWhileNode } };
use utils::guard_none;

use crate::{ parser::{ LoopArgument, Parser }, utils::{ match_pattern, Lookup } };

use super::block::BlockParser;

#[derive(Debug, Clone)]
pub struct DoWhileParser {}

impl DoWhileParser {
  pub fn test(tokens: &Vec<Token>, _: &mut LoopArgument) -> Option<Vec<Vec<Token>>> {
    match_pattern(tokens, [Lookup::Equal(vec![TokenType::Do])].to_vec())
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<Vec<Token>>,
    _: &mut LoopArgument
  ) -> Option<Box<Node>> {
    if let [_] = matched.as_slice() {
      let body = BlockParser::new(parser);
      parser.position += 2;
      let condition = guard_none!(
        parser.get_statement(
          &mut LoopArgument::with_tokens("do_while", &[], &[TokenType::RightParenthesis])
        )
      );
      parser.position += 1;
      return Some(DoWhileNode::new(condition, body));
    }
    None
  }
}