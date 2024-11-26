use backyard_lexer::token::{ Token, TokenType };
use backyard_nodes::node::{ Node, ConstNode, ConstPropertyNode };

use crate::{
  error::ParserError,
  parser::{ LoopArgument, Parser },
  utils::{ match_pattern, some_or_default, Lookup },
};

use super::{ assignment::AssignmentParser, comment::CommentParser, identifier::IdentifierParser };

#[derive(Debug, Clone)]
pub struct ConstParser;

impl ConstParser {
  pub fn get_consts(parser: &mut Parser) -> Result<Vec<Box<Node>>, ParserError> {
    let consts = parser.get_children(
      &mut LoopArgument::new(
        "const",
        &[TokenType::Comma],
        &[TokenType::Semicolon],
        &[
          (IdentifierParser::test, IdentifierParser::parse),
          (AssignmentParser::test, AssignmentParser::parse),
          (CommentParser::test, CommentParser::parse),
        ]
      )
    );
    parser.position -= 1;
    consts
  }
}

impl ConstParser {
  pub fn test(tokens: &[Token], _: &mut LoopArgument) -> Option<Vec<Vec<Token>>> {
    match_pattern(tokens, [Lookup::Equal(vec![TokenType::Const])].to_vec())
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<Vec<Token>>,
    args: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [_] = matched.as_slice() {
      return Ok(ConstNode::new(ConstParser::get_consts(parser)?));
    }
    Err(ParserError::internal("Const", args))
  }
}

#[derive(Debug, Clone)]
pub struct ConstPropertyParser;

impl ConstPropertyParser {
  pub fn test(tokens: &[Token], _: &mut LoopArgument) -> Option<Vec<Vec<Token>>> {
    match_pattern(
      tokens,
      [
        Lookup::Optional(vec![TokenType::Public, TokenType::Private, TokenType::Protected]),
        Lookup::Equal(vec![TokenType::Const]),
      ].to_vec()
    )
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<Vec<Token>>,
    args: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [visibility, _] = matched.as_slice() {
      return Ok(
        ConstPropertyNode::new(
          some_or_default(visibility.first(), String::from(""), |i| i.value.to_owned()),
          ConstParser::get_consts(parser)?
        )
      );
    }
    Err(ParserError::internal("ConstProperty", args))
  }
}
