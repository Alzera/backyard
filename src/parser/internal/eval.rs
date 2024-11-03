use crate::{
  lexer::token::{ Token, TokenType },
  parser::{
    node::Node,
    nodes::eval::EvalNode,
    parser::{ Internal, LoopArgument, Parser },
    utils::{ match_pattern, Lookup },
  },
};

#[derive(Debug, Clone)]
pub struct EvalParser {}

impl Internal for EvalParser {
  fn test(&self, tokens: &Vec<Token>, _: &LoopArgument) -> Option<Vec<Vec<Token>>> {
    match_pattern(
      tokens,
      [
        Lookup::Equal(vec![TokenType::Eval]),
        Lookup::Equal(vec![TokenType::LeftParenthesis]),
      ].to_vec()
    )
  }

  fn parse(&self, parser: &mut Parser, matched: Vec<Vec<Token>>, _: &LoopArgument) -> Option<Node> {
    if let [_, _] = matched.as_slice() {
      let argument = parser.get_statement(
        &mut LoopArgument::with_tokens("eval", &[TokenType::RightParenthesis], &[])
      );
      if argument.is_none() {
        return None;
      }
      return Some(EvalNode::new(argument.unwrap()));
    }
    None
  }
}
