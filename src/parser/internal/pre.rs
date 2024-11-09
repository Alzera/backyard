use crate::{
  lexer::token::{ Token, TokenType },
  parser::{
    node::Node,
    nodes::pre::PreNode,
    parser::{ LoopArgument, Parser },
    utils::{ match_pattern, Lookup },
  },
};

#[derive(Debug, Clone)]
pub struct PreParser {}

impl PreParser {
  pub fn test(tokens: &Vec<Token>, _: &mut LoopArgument) -> Option<Vec<Vec<Token>>> {
    match_pattern(
      tokens,
      [
        Lookup::Equal(
          vec![TokenType::PreIncrement, TokenType::PreDecrement, TokenType::BooleanNegate]
        ),
      ].to_vec()
    )
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<Vec<Token>>,
    _: &mut LoopArgument
  ) -> Option<Node> {
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
      return Some(PreNode::new(argument.unwrap(), operator.unwrap().value.to_owned()));
    }
    None
  }
}
