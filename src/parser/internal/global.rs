use crate::{
  lexer::token::{ Token, TokenType },
  parser::{
    node::Node,
    nodes::global::GlobalNode,
    parser::{ LoopArgument, Parser },
    utils::{ match_pattern, Lookup },
  },
};

use super::{ comment::CommentParser, variable::VariableParser };

#[derive(Debug, Clone)]
pub struct GlobalParser {}

impl GlobalParser {
  pub fn test(tokens: &Vec<Token>, _: &mut LoopArgument) -> Option<Vec<Vec<Token>>> {
    match_pattern(
      tokens,
      [
        Lookup::Equal(vec![TokenType::Global]),
        Lookup::Equal(vec![TokenType::Variable, TokenType::VariableBracketOpen]),
      ].to_vec()
    )
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<Vec<Token>>,
    _: &mut LoopArgument
  ) -> Option<Node> {
    if let [_, _] = matched.as_slice() {
      parser.position -= 1;
      if
        let Some(variable) = parser.get_statement(
          &mut LoopArgument::new(
            "global",
            &[],
            &[TokenType::Semicolon],
            &[
              (VariableParser::test, VariableParser::parse),
              (CommentParser::test, CommentParser::parse),
            ]
          )
        )
      {
        return Some(GlobalNode::new(variable));
      }
    }
    None
  }
}
