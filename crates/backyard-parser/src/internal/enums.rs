use backyard_lexer::token::{ Token, TokenType };
use backyard_nodes::node::{ EnumItemNode, EnumNode, Location, Node };

use crate::{
  error::ParserError,
  parser::{ LoopArgument, Parser },
  utils::{ match_pattern, Lookup, LookupResult, LookupResultWrapper },
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
  pub fn test(tokens: &[Token], _: &mut LoopArgument) -> Option<Vec<LookupResult>> {
    match_pattern(
      tokens,
      &[
        Lookup::Equal(&[TokenType::Enum]),
        Lookup::Equal(&[TokenType::Identifier]),
        Lookup::Optional(&[TokenType::Colon]),
        Lookup::OptionalType,
        Lookup::Optional(&[TokenType::Implements]),
        Lookup::Optional(&[TokenType::Identifier, TokenType::Name]),
        Lookup::Equal(&[TokenType::LeftCurlyBracket]),
      ]
    )
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<LookupResult>,
    start_loc: Location,
    _: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [_, name, _, enum_type, has_implements, implements, _] = matched.as_slice() {
      let enum_type = if
        let LookupResultWrapper::OptionalType(enum_type) = enum_type.wrapper.to_owned()
      {
        enum_type
      } else {
        return Err(ParserError::Internal);
      };
      let name = if let LookupResultWrapper::Equal(name) = &name.wrapper {
        IdentifierParser::from_token(name)
      } else {
        return Err(ParserError::Internal);
      };
      let implements = if !has_implements.is_empty() {
        if let LookupResultWrapper::Optional(Some(implements)) = &implements.wrapper {
          Some(IdentifierParser::from_token(implements))
        } else {
          None
        }
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
      return Ok(EnumNode::loc(name, enum_type, implements, items, parser.gen_loc(start_loc)));
    }
    Err(ParserError::Internal)
  }
}

#[derive(Debug, Clone)]
pub struct EnumItemParser;

impl EnumItemParser {
  pub fn test(tokens: &[Token], _: &mut LoopArgument) -> Option<Vec<LookupResult>> {
    match_pattern(tokens, &[Lookup::Equal(&[TokenType::Case])])
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<LookupResult>,
    start_loc: Location,
    _: &mut LoopArgument
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
        return Ok(EnumItemNode::loc(value, parser.gen_loc(start_loc)));
      }
    }
    Err(ParserError::Internal)
  }
}
