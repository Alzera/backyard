use backyard_lexer::token::{ Token, TokenType };
use backyard_nodes::node::{ EnumItemNode, EnumNode, Node };

use crate::{
  error::ParserError,
  parser::{ LoopArgument, Parser },
  utils::{ match_pattern, Lookup },
};

use super::{ comment::CommentParser, identifier::IdentifierParser };

#[derive(Debug, Clone)]
pub struct EnumParser {}

impl EnumParser {
  pub fn test(tokens: &Vec<Token>, _: &mut LoopArgument) -> Option<Vec<Vec<Token>>> {
    match_pattern(
      tokens,
      [
        Lookup::Equal(vec![TokenType::Enum]),
        Lookup::Equal(vec![TokenType::Identifier]),
        Lookup::Equal(vec![TokenType::LeftCurlyBracket]),
      ].to_vec()
    )
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<Vec<Token>>,
    args: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [_, name, _] = matched.as_slice() {
      let items = parser.get_children(
        &mut LoopArgument::new(
          "enum",
          &[TokenType::Semicolon],
          &[TokenType::RightCurlyBracket],
          &[
            (EnumItemParser::test, EnumItemParser::parse),
            (CommentParser::test, CommentParser::parse),
          ]
        )
      )?;
      return Ok(EnumNode::new(IdentifierParser::from_matched(name), items));
    }
    Err(ParserError::internal("Enum", args))
  }
}

#[derive(Debug, Clone)]
pub struct EnumItemParser {}

impl EnumItemParser {
  pub fn test(tokens: &Vec<Token>, _: &mut LoopArgument) -> Option<Vec<Vec<Token>>> {
    match_pattern(tokens, [Lookup::Equal(vec![TokenType::Case])].to_vec())
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<Vec<Token>>,
    args: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [_] = matched.as_slice() {
      if
        let Some(value) = parser.get_statement(
          &mut LoopArgument::with_tokens(
            "enum_item",
            &[],
            &[TokenType::Semicolon, TokenType::RightSquareBracket]
          )
        )?
      {
        return Ok(EnumItemNode::new(value));
      }
    }
    Err(ParserError::internal("EnumItem", args))
  }
}
