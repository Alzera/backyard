use crate::{
  lexer::token::{ Token, TokenType },
  parser::{
    node::{ EncapsedNode, EncapsedPartNode, Node, Nodes, StringNode },
    parser::{ Internal, LoopArgument, Parser },
  },
};

use super::variable::VariableParser;

#[derive(Debug, Clone)]
pub struct StringParser {}

impl Internal for StringParser {
  fn test(&self, tokens: &Vec<Token>, _: &LoopArgument) -> Option<Vec<Vec<Token>>> {
    if let Some(token) = tokens.get(0) {
      if [TokenType::EncapsedStringOpen, TokenType::String].contains(&token.token_type) {
        return Some(vec![vec![token.to_owned()]]);
      }
    }
    None
  }

  fn parse(&self, parser: &mut Parser, matched: Vec<Vec<Token>>, _: &LoopArgument) -> Option<Node> {
    if let [string_type] = matched.as_slice() {
      if let Some(string_type) = string_type.get(0) {
        if string_type.token_type == TokenType::EncapsedStringOpen {
          return StringParser::parse_encapsed(parser);
        } else if string_type.token_type == TokenType::String {
          return Some(
            Box::new(StringNode {
              value: string_type.value.to_owned(),
            })
          );
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
          values.push(
            Box::new(EncapsedPartNode {
              is_advanced: false,
              value: Box::new(StringNode { value: i.value.to_owned() }),
            })
          ),
        TokenType::Variable =>
          values.push(
            Box::new(EncapsedPartNode {
              is_advanced: false,
              value: VariableParser::new(i.value.to_owned(), false),
            })
          ),
        TokenType::AdvanceInterpolationOpen => {
          let value = parser.get_statement(
            &mut LoopArgument::with_tokens("string", &[TokenType::AdvanceInterpolationClose], &[])
          );
          parser.position += 1;
          if value.is_none() {
            continue;
          }
          values.push(
            Box::new(EncapsedPartNode {
              is_advanced: true,
              value: value.unwrap(),
            })
          );
        }
        _ => {
          continue;
        }
      }
    }
    return Some(
      Box::new(EncapsedNode {
        values,
      })
    );
  }
}
