use crate::{
  lexer::token::{ Token, TokenType },
  parser::{
    node::{ GlobalNode, Node },
    parser::{ Internal, LoopArgument, Parser, ParserInternal },
    utils::{ match_pattern, Lookup },
  },
};

use super::variable::VariableParser;

#[derive(Debug, Clone)]
pub struct GlobalParser {}

impl Internal for GlobalParser {
  fn test(&self, tokens: &Vec<Token>, _: &LoopArgument) -> Option<Vec<Vec<Token>>> {
    match_pattern(
      tokens,
      [
        Lookup::Equal(vec![TokenType::Global]),
        Lookup::Equal(vec![TokenType::Variable, TokenType::VariableBracketOpen]),
      ].to_vec()
    )
  }

  fn parse(&self, parser: &mut Parser, matched: Vec<Vec<Token>>, _: &LoopArgument) -> Option<Node> {
    if let [_, _] = matched.as_slice() {
      parser.position -= 1;
      if
        let Some(variable) = parser.get_statement(
          &mut LoopArgument::new(
            "global",
            &[],
            &[TokenType::Semicolon],
            &[ParserInternal::Variable(VariableParser {})]
          )
        )
      {
        return Some(Box::new(GlobalNode { variable }));
      }
    }
    None
  }
}
