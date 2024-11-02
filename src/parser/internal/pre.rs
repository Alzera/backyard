use crate::{
  lexer::token::{ Token, TokenType },
  parser::{
    node::{ Node, PreNode },
    parser::{ Internal, LoopArgument, Parser },
    utils::{ match_pattern, Lookup },
  },
};

#[derive(Debug, Clone)]
pub struct PreParser {}

impl Internal for PreParser {
  fn test(&self, tokens: &Vec<Token>, _: &LoopArgument) -> Option<Vec<Vec<Token>>> {
    match_pattern(
      tokens,
      [
        Lookup::Equal(
          vec![TokenType::PreIncrement, TokenType::PreDecrement, TokenType::BooleanNegate]
        ),
      ].to_vec()
    )
  }

  fn parse(&self, parser: &mut Parser, matched: Vec<Vec<Token>>, _: &LoopArgument) -> Option<Node> {
    if let [operator] = matched.as_slice() {
      let operator = operator.get(0);
      if operator.is_none() {
        return None;
      }
      let argument = parser.get_statement(
        &mut LoopArgument::with_tokens("pre", &[TokenType::Semicolon], &[])
      );
      if argument.is_none() {
        return None;
      }
      return Some(
        Box::new(PreNode {
          variable: argument.unwrap(),
          operator: operator.unwrap().value.to_owned(),
        })
      );
    }
    None
  }
}
