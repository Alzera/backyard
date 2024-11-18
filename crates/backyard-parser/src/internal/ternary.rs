use backyard_lexer::token::{ Token, TokenType, TokenTypeArrayCombine };
use backyard_nodes::{ node::{ Node, TernaryNode } };

use crate::{ parser::{ LoopArgument, Parser }, utils::{ match_pattern, Lookup } };

#[derive(Debug, Clone)]
pub struct TernaryParser {}

impl TernaryParser {
  pub fn test(tokens: &Vec<Token>, _: &mut LoopArgument) -> Option<Vec<Vec<Token>>> {
    match_pattern(tokens, [Lookup::Equal(vec![TokenType::QuestionMark])].to_vec())
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<Vec<Token>>,
    args: &mut LoopArgument
  ) -> Option<Box<Node>> {
    if let [_] = matched.as_slice() {
      if args.last_expr.is_none() {
        return None;
      }
      let valid = parser.get_statement(
        &mut LoopArgument::with_tokens("ternary_valid", &[], &[TokenType::Colon])
      );
      if valid.is_none() {
        return None;
      }
      parser.position += 1;
      let invalid = parser.get_statement(
        &mut LoopArgument::with_tokens(
          "ternary_invalid",
          &args.separators.combine(&[TokenType::Semicolon]),
          &args.breakers
        )
      );
      if invalid.is_none() {
        return None;
      }
      return Some(
        TernaryNode::new(args.last_expr.to_owned().unwrap(), valid.unwrap(), invalid.unwrap())
      );
    }
    None
  }
}