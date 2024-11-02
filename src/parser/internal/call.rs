use crate::{
  lexer::token::{ Token, TokenType },
  parser::{
    node::{ ArgumentNode, CallNode, Node, Nodes },
    parser::{ Internal, LoopArgument, Parser, ParserInternal },
    utils::{ match_pattern, Lookup },
  },
};

use super::identifier::IdentifierParser;

#[derive(Debug, Clone)]
pub struct CallParser {}

impl CallParser {
  pub fn get_arguments(parser: &mut Parser) -> Nodes {
    parser.get_children(
      &mut LoopArgument::new(
        "call",
        &[TokenType::Comma],
        &[TokenType::RightParenthesis],
        &[ParserInternal::Argument(ArgumentParser {})]
      )
    )
  }
}

impl Internal for CallParser {
  fn test(&self, tokens: &Vec<Token>, _: &LoopArgument) -> Option<Vec<Vec<Token>>> {
    match_pattern(
      tokens,
      [
        Lookup::Equal(vec![TokenType::Identifier]),
        Lookup::Equal(vec![TokenType::LeftParenthesis]),
      ].to_vec()
    )
  }

  fn parse(&self, parser: &mut Parser, matched: Vec<Vec<Token>>, _: &LoopArgument) -> Option<Node> {
    if let [name, _] = matched.as_slice() {
      if let Some(name) = name.get(0) {
        return Some(
          Box::new(CallNode {
            name: IdentifierParser::new(name.value.to_owned()),
            arguments: CallParser::get_arguments(parser),
          })
        );
      }
    }
    None
  }
}

#[derive(Debug, Clone)]
pub struct ArgumentParser {}

impl Internal for ArgumentParser {
  fn test(&self, tokens: &Vec<Token>, _: &LoopArgument) -> Option<Vec<Vec<Token>>> {
    match_pattern(
      tokens,
      [
        Lookup::Optional(vec![TokenType::Identifier]),
        Lookup::Optional(vec![TokenType::Colon]),
      ].to_vec()
    )
  }

  fn parse(&self, parser: &mut Parser, matched: Vec<Vec<Token>>, _: &LoopArgument) -> Option<Node> {
    // println!("ArgumentNode::parse: {:?}", matched);
    if let [name, _] = matched.as_slice() {
      let value = parser.get_statement(
        &mut LoopArgument::with_tokens(
          "argument",
          &[TokenType::Comma, TokenType::RightParenthesis],
          &[]
        )
      );
      if value.is_none() {
        return None;
      }
      let name = match name.len() {
        1 => Some(IdentifierParser::from_matched(name)),
        _ => None,
      };
      return Some(
        Box::new(ArgumentNode {
          name,
          value: value.unwrap(),
        })
      );
    }
    None
  }
}
