use backyard_lexer::token::{ Token, TokenType };
use backyard_nodes::node::{ EnumItemNode, EnumNode, Location, Node };

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
      &[
        Lookup::Equal(&[TokenType::Enum]),
        Lookup::Equal(&[TokenType::Identifier]),
        Lookup::Optional(&[TokenType::Colon]),
        Lookup::Optional(&[TokenType::Type]),
        Lookup::Optional(&[TokenType::Implements]),
        Lookup::Optional(&[TokenType::Identifier, TokenType::Name]),
        Lookup::Equal(&[TokenType::LeftCurlyBracket]),
      ]
    )
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<Vec<Token>>,
    start_loc: Location,
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
      return Ok(
        EnumNode::new(
          IdentifierParser::from_matched(name),
          enum_type,
          implements,
          items,
          parser.gen_loc(start_loc)
        )
      );
    }
    Err(ParserError::internal("Enum", args))
  }
}

#[derive(Debug, Clone)]
pub struct EnumItemParser;

impl EnumItemParser {
  pub fn test(tokens: &[Token], _: &mut LoopArgument) -> Option<Vec<Vec<Token>>> {
    match_pattern(tokens, &[Lookup::Equal(&[TokenType::Case])])
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<Vec<Token>>,
    start_loc: Location,
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
        return Ok(EnumItemNode::new(value, parser.gen_loc(start_loc)));
      }
    }
    Err(ParserError::internal("EnumItem", args))
  }
}
