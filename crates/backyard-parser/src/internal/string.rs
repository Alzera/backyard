use backyard_lexer::token::{ Token, TokenType };
use backyard_nodes::node::{ Node, EncapsedNode, EncapsedPartNode, StringNode };
use utils::guard;

use crate::{ error::ParserError, parser::{ LoopArgument, Parser } };

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
    args: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [string_type] = matched.as_slice() {
      if let Some(string_type) = string_type.get(0) {
        if string_type.token_type == TokenType::EncapsedStringOpen {
          return StringParser::parse_encapsed(parser);
        } else if string_type.token_type == TokenType::String {
          return Ok(StringNode::new(string_type.value.to_owned()));
        }
      }
    }
    Err(ParserError::internal("String", args))
  }
}

impl StringParser {
  fn parse_encapsed(parser: &mut Parser) -> Result<Box<Node>, ParserError> {
    let mut values: Vec<Box<Node>> = vec![];
    while let Some(i) = parser.tokens.get(parser.position) {
      parser.position += 1;
      match i.token_type {
        TokenType::EncapsedStringClose => {
          break;
        }
        TokenType::EncapsedString =>
          values.push(EncapsedPartNode::new(false, StringNode::new(i.value.to_owned()))),
        TokenType::Variable =>
          values.push(EncapsedPartNode::new(false, VariableParser::new(i.value.to_owned(), false))),
        TokenType::AdvanceInterpolationOpen => {
          let value = guard!(
            parser.get_statement(
              &mut LoopArgument::with_tokens("string", &[TokenType::AdvanceInterpolationClose], &[])
            )?,
            {
              continue;
            }
          );
          parser.position += 1;
          values.push(EncapsedPartNode::new(true, value));
        }
        _ => {
          continue;
        }
      }
    }
    return Ok(EncapsedNode::new(values));
  }
}
