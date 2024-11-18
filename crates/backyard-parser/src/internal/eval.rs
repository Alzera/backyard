use backyard_lexer::token::{ Token, TokenType };
use backyard_nodes::{ node::{ Node, EvalNode } };

use crate::{ parser::{ LoopArgument, Parser }, utils::{ match_pattern, Lookup } };

#[derive(Debug, Clone)]
pub struct EvalParser {}

impl EvalParser {
  pub fn test(tokens: &Vec<Token>, _: &mut LoopArgument) -> Option<Vec<Vec<Token>>> {
    match_pattern(
      tokens,
      [
        Lookup::Equal(vec![TokenType::Eval]),
        Lookup::Equal(vec![TokenType::LeftParenthesis]),
      ].to_vec()
    )
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<Vec<Token>>,
    _: &mut LoopArgument
  ) -> Option<Box<Node>> {
    if let [_, _] = matched.as_slice() {
      let argument = parser.get_statement(
        &mut LoopArgument::with_tokens("eval", &[TokenType::RightParenthesis], &[])
      );
      parser.position += 1;
      if argument.is_none() {
        return None;
      }
      return Some(EvalNode::new(argument.unwrap()));
    }
    None
  }
}
