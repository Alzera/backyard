use crate::{
  lexer::token::{ Token, TokenType },
  parser::{
    node::{ Node, Nodes },
    nodes::string::{ EncapsedNode, EncapsedPartNode, StringNode },
    parser::{ LoopArgument, Parser },
  },
};

use super::variable::VariableParser;

#[derive(Debug, Clone)]
pub struct StringParser {}

impl StringParser {
  pub fn test(tokens: &Vec<Token>, _: &mut LoopArgument) -> Option<Vec<Vec<Token>>> {
    if let Some(token) = tokens.get(0) {
      if [TokenType::EncapsedStringOpen, TokenType::String].contains(&token.token_type) {
        return Some(vec![vec![token.to_owned()]]);
      }
    }
    None
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<Vec<Token>>,
    _: &mut LoopArgument
  ) -> Option<Node> {
    if let [string_type] = matched.as_slice() {
      if let Some(string_type) = string_type.get(0) {
        if string_type.token_type == TokenType::EncapsedStringOpen {
          return StringParser::parse_encapsed(parser);
        } else if string_type.token_type == TokenType::String {
          return Some(StringNode::boxed(string_type.value.to_owned()));
        }
      }
    }
    None
  }
}

impl StringParser {
  fn parse_encapsed(parser: &mut Parser) -> Option<Node> {
    let mut values: Nodes = vec![];
    while let Some(i) = parser.tokens.get(parser.position) {
      parser.position += 1;
      match i.token_type {
        TokenType::EncapsedStringClose => {
          break;
        }
        TokenType::EncapsedString =>
          values.push(EncapsedPartNode::boxed(false, StringNode::boxed(i.value.to_owned()))),
        TokenType::Variable =>
          values.push(
            EncapsedPartNode::boxed(false, VariableParser::new(i.value.to_owned(), false))
          ),
        TokenType::AdvanceInterpolationOpen => {
          let value = parser.get_statement(
            &mut LoopArgument::with_tokens("string", &[TokenType::AdvanceInterpolationClose], &[])
          );
          parser.position += 1;
          if value.is_none() {
            continue;
          }
          values.push(EncapsedPartNode::boxed(true, value.unwrap()));
        }
        _ => {
          continue;
        }
      }
    }
    return Some(EncapsedNode::boxed(values));
  }
}
