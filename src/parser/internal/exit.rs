use crate::{
  lexer::token::{ Token, TokenType },
  parser::{
    node::Node,
    nodes::exit::ExitNode,
    parser::{ LoopArgument, Parser },
    utils::{ match_pattern, Lookup },
  },
};

#[derive(Debug, Clone)]
pub struct ExitParser {}

impl ExitParser {
  pub fn test(tokens: &Vec<Token>, _: &LoopArgument) -> Option<Vec<Vec<Token>>> {
    match_pattern(
      tokens,
      [
        Lookup::Equal(vec![TokenType::Exit, TokenType::Die]),
        Lookup::Equal(vec![TokenType::LeftParenthesis]),
      ].to_vec()
    )
  }

  pub fn parse(parser: &mut Parser, matched: Vec<Vec<Token>>, _: &LoopArgument) -> Option<Node> {
    if let [_, _] = matched.as_slice() {
      let argument = parser.get_statement(
        &mut LoopArgument::with_tokens("exit", &[TokenType::RightParenthesis], &[])
      );
      if argument.is_none() {
        return None;
      }
      return Some(ExitNode::new(argument.unwrap()));
    }
    None
  }
}
