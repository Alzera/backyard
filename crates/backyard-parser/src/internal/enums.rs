use backyard_lexer::token::{ Token, TokenType };
use backyard_nodes::node::{ EnumItemNode, EnumNode, Node };

use crate::{
  error::ParserError,
  parser::{ LoopArgument, Parser },
  utils::{ match_pattern, Lookup },
};

use super::{
  attribute::AttributeParser,
  comment::CommentParser,
  consts::ConstPropertyParser,
  identifier::IdentifierParser,
  method::MethodParser,
};

#[derive(Debug, Clone)]
pub struct EnumParser;

impl EnumParser {
  pub fn test(tokens: &[Token], _: &mut LoopArgument) -> Option<Vec<Vec<Token>>> {
    match_pattern(
      tokens,
      [
        Lookup::Equal(vec![TokenType::Enum]),
        Lookup::Equal(vec![TokenType::Identifier]),
        Lookup::Optional(vec![TokenType::Colon]),
        Lookup::Optional(vec![TokenType::Type]),
        Lookup::Optional(vec![TokenType::Implements]),
        Lookup::Optional(vec![TokenType::Identifier, TokenType::Name]),
        Lookup::Equal(vec![TokenType::LeftCurlyBracket]),
      ].to_vec()
    )
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<Vec<Token>>,
    args: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [_, name, has_type, enum_type, has_implements, implements, _] = matched.as_slice() {
      let enum_type = if !has_type.is_empty() {
        Some(IdentifierParser::from_matched(enum_type))
      } else {
        None
      };
      let implements = if !has_implements.is_empty() {
        Some(IdentifierParser::from_matched(implements))
      } else {
        None
      };
      let items = parser.get_children(
        &mut LoopArgument::new(
          "enum",
          &[TokenType::Semicolon],
          &[TokenType::RightCurlyBracket],
          &[
            (MethodParser::test, MethodParser::parse),
            (ConstPropertyParser::test, ConstPropertyParser::parse),
            (EnumItemParser::test, EnumItemParser::parse),
            (AttributeParser::test, AttributeParser::parse),
            (CommentParser::test, CommentParser::parse),
          ]
        )
      )?;
      return Ok(EnumNode::new(IdentifierParser::from_matched(name), enum_type, implements, items));
    }
    Err(ParserError::internal("Enum", args))
  }
}

#[derive(Debug, Clone)]
pub struct EnumItemParser;

impl EnumItemParser {
  pub fn test(tokens: &[Token], _: &mut LoopArgument) -> Option<Vec<Vec<Token>>> {
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
