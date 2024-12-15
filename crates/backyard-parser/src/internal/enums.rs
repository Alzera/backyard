use backyard_lexer::token::TokenType;
use backyard_nodes::{
  EnumItemNode,
  EnumNode,
  Location,
  Node,
  utils::{ IntoBoxedNode, IntoBoxedOptionNode },
};

use crate::{
  error::ParserError,
  parser::{ LoopArgument, Parser },
  utils::{ match_pattern, Lookup, LookupResult },
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
  pub fn test<'arena, 'a>(
    parser: &mut Parser<'arena, 'a>,
    _: &mut LoopArgument
  ) -> Option<std::vec::Vec<LookupResult<'arena>>> {
    match_pattern(
      parser,
      &[
        Lookup::Equal(&[TokenType::Enum]),
        Lookup::Equal(&[TokenType::UnqualifiedName]),
        Lookup::Optional(&[TokenType::Colon]),
        Lookup::OptionalType,
        Lookup::Optional(&[TokenType::Implements]),
        Lookup::Optional(
          &[
            TokenType::UnqualifiedName,
            TokenType::QualifiedName,
            TokenType::RelativeName,
            TokenType::FullyQualifiedName,
          ]
        ),
        Lookup::Equal(&[TokenType::LeftCurlyBracket]),
      ]
    )
  }

  pub fn parse<'arena, 'a, 'b>(
    parser: &mut Parser<'arena, 'a>,
    mut matched: std::vec::Vec<LookupResult<'arena>>,
    start_loc: Location,
    _: &mut LoopArgument<'arena, 'b>
  ) -> Result<Node<'arena>, ParserError> {
    if let [_, name, _, enum_type, has_implements, implements, _] = matched.as_mut_slice() {
      let name = IdentifierParser::from_token(name.as_equal(parser)?);
      let implements = has_implements
        .as_optional(parser)
        .map(|__construct| implements.as_optional(parser).map(IdentifierParser::from_token))
        .unwrap_or_default();
      let items = parser.get_children(
        &mut LoopArgument::new(
          parser.arena,
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
        EnumNode::loc(
          name.into_boxed(parser.arena),
          enum_type.as_optional_type().into_boxed(parser.arena),
          implements.into_boxed(parser.arena),
          items,
          parser.gen_loc(start_loc)
        )
      );
    }
    Err(ParserError::Internal)
  }
}

#[derive(Debug, Clone)]
pub struct EnumItemParser;

impl EnumItemParser {
  pub fn test<'arena, 'a>(
    parser: &mut Parser<'arena, 'a>,
    _: &mut LoopArgument
  ) -> Option<std::vec::Vec<LookupResult<'arena>>> {
    match_pattern(parser, &[Lookup::Equal(&[TokenType::Case])])
  }

  pub fn parse<'arena, 'a, 'b>(
    parser: &mut Parser<'arena, 'a>,
    matched: std::vec::Vec<LookupResult<'arena>>,
    start_loc: Location,
    _: &mut LoopArgument<'arena, 'b>
  ) -> Result<Node<'arena>, ParserError> {
    if let [_] = matched.as_slice() {
      if
        let Some(value) = parser.get_statement(
          &mut LoopArgument::with_tokens(
            parser.arena,
            "enum_item",
            &[],
            &[TokenType::Semicolon, TokenType::RightSquareBracket]
          )
        )?
      {
        return Ok(EnumItemNode::loc(value.into_boxed(parser.arena), parser.gen_loc(start_loc)));
      }
    }
    Err(ParserError::Internal)
  }
}
