use crate::{
  lexer::token::{ Token, TokenType },
  parser::{
    node::{ Node, Nodes },
    nodes::call::{ ArgumentNode, CallNode },
    parser::{ LoopArgument, Parser },
    utils::{ match_pattern, Lookup },
  },
};

use super::{ comment::CommentParser, identifier::IdentifierParser };

#[derive(Debug, Clone)]
pub struct CallParser {}

impl CallParser {
  pub fn get_arguments(parser: &mut Parser) -> Nodes {
    parser.get_children(
      &mut LoopArgument::new(
        "call",
        &[TokenType::Comma],
        &[TokenType::RightParenthesis],
        &[
          (ArgumentParser::test, ArgumentParser::parse),
          (CommentParser::test, CommentParser::parse),
        ]
      )
    )
  }
}

impl CallParser {
  pub fn test(tokens: &Vec<Token>, _: &LoopArgument) -> Option<Vec<Vec<Token>>> {
    match_pattern(
      tokens,
      [
        Lookup::Equal(vec![TokenType::Identifier]),
        Lookup::Equal(vec![TokenType::LeftParenthesis]),
      ].to_vec()
    )
  }

  pub fn parse(parser: &mut Parser, matched: Vec<Vec<Token>>, _: &LoopArgument) -> Option<Node> {
    if let [name, _] = matched.as_slice() {
      if let Some(name) = name.get(0) {
        return Some(
          CallNode::new(
            IdentifierParser::new(name.value.to_owned()),
            CallParser::get_arguments(parser)
          )
        );
      }
    }
    None
  }
}

#[derive(Debug, Clone)]
pub struct ArgumentParser {}

impl ArgumentParser {
  pub fn test(tokens: &Vec<Token>, _: &LoopArgument) -> Option<Vec<Vec<Token>>> {
    match_pattern(
      tokens,
      [
        Lookup::Optional(vec![TokenType::Identifier]),
        Lookup::Optional(vec![TokenType::Colon]),
      ].to_vec()
    )
  }

  pub fn parse(parser: &mut Parser, matched: Vec<Vec<Token>>, _: &LoopArgument) -> Option<Node> {
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
      return Some(ArgumentNode::new(name, value.unwrap()));
    }
    None
  }
}
