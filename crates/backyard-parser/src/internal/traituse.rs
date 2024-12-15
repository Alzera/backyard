use bumpalo::vec;
use backyard_lexer::token::TokenType;
use backyard_nodes::{
  Location,
  Node,
  TraitUseAliasNode,
  TraitUseNode,
  TraitUsePrecedenceNode,
  Visibility,
  utils::{ IntoBoxedNode, IntoBoxedOptionNode },
};

use crate::{
  error::ParserError,
  guard,
  parser::{ LoopArgument, Parser },
  utils::{ match_pattern, Lookup, LookupResult },
};

use super::{ comment::CommentParser, identifier::IdentifierParser };

#[derive(Debug, Clone)]
pub struct TraitUseParser;

impl TraitUseParser {
  pub fn test<'arena, 'a>(
    parser: &mut Parser<'arena, 'a>,
    _: &mut LoopArgument
  ) -> Option<std::vec::Vec<LookupResult<'arena>>> {
    match_pattern(parser, &[Lookup::Equal(&[TokenType::Use])])
  }

  pub fn parse<'arena, 'a, 'b>(
    parser: &mut Parser<'arena, 'a>,
    matched: std::vec::Vec<LookupResult<'arena>>,
    start_loc: Location,
    _: &mut LoopArgument<'arena, 'b>
  ) -> Result<Node<'arena>, ParserError> {
    if let [_] = matched.as_slice() {
      let traits = parser.get_children(
        &mut LoopArgument::new(
          parser.arena,
          "traituse",
          &[TokenType::Comma],
          &[TokenType::Semicolon, TokenType::LeftCurlyBracket],
          &[
            (IdentifierParser::test, IdentifierParser::parse),
            (CommentParser::test, CommentParser::parse),
          ]
        )
      )?;
      let mut adaptations = vec![in parser.arena];
      if guard!(parser.tokens.get(parser.position - 1)).token_type == TokenType::Semicolon {
        parser.position -= 1;
      } else {
        adaptations = parser.get_children(
          &mut LoopArgument::new(
            parser.arena,
            "traituse_body",
            &[TokenType::Semicolon],
            &[TokenType::RightCurlyBracket],
            &[
              (TraitUseAliasParser::test, TraitUseAliasParser::parse),
              (TraitUsePrecedenceParser::test, TraitUsePrecedenceParser::parse),
              (CommentParser::test, CommentParser::parse),
            ]
          )
        )?;
      }
      return Ok(TraitUseNode::loc(traits, adaptations, parser.gen_loc(start_loc)));
    }
    Err(ParserError::Internal)
  }
}

#[derive(Debug, Clone)]
pub struct TraitUseAliasParser;

impl TraitUseAliasParser {
  pub fn test<'arena, 'a>(
    parser: &mut Parser<'arena, 'a>,
    _: &mut LoopArgument
  ) -> Option<std::vec::Vec<LookupResult<'arena>>> {
    match_pattern(
      parser,
      &[
        Lookup::Equal(&[TokenType::Identifier, TokenType::Name, TokenType::Get, TokenType::Set]),
        Lookup::Optional(&[TokenType::DoubleColon]),
        Lookup::Optional(&[TokenType::Identifier, TokenType::Name, TokenType::Get, TokenType::Set]),
        Lookup::Equal(&[TokenType::As]),
        Lookup::Optional(&[TokenType::Public, TokenType::Private, TokenType::Protected]),
        Lookup::Optional(&[TokenType::Identifier, TokenType::Name, TokenType::Get, TokenType::Set]),
      ]
    )
  }

  pub fn parse<'arena, 'a, 'b>(
    parser: &mut Parser<'arena, 'a>,
    matched: std::vec::Vec<LookupResult<'arena>>,
    start_loc: Location,
    _: &mut LoopArgument<'arena, 'b>
  ) -> Result<Node<'arena>, ParserError> {
    if let [trait_name, double_colon, name, _, visibility, alias] = matched.as_slice() {
      let trait_name = IdentifierParser::from_token(trait_name.as_equal(parser)?);
      let name = name.as_optional(parser).map(IdentifierParser::from_token);
      let (trait_name_parsed, name_parsed) = if !double_colon.is_empty() {
        (Some(trait_name), name.unwrap())
      } else {
        (None, trait_name)
      };
      let alias = alias.as_optional(parser).map(IdentifierParser::from_token);
      let visibility = visibility
        .as_optional(parser)
        .and_then(|x| Visibility::try_from(&x.value).ok());
      return Ok(
        TraitUseAliasNode::loc(
          trait_name_parsed.into_boxed(parser.arena),
          name_parsed.into_boxed(parser.arena),
          alias.into_boxed(parser.arena),
          visibility,
          parser.gen_loc(start_loc)
        )
      );
    }
    Err(ParserError::Internal)
  }
}

#[derive(Debug, Clone)]
pub struct TraitUsePrecedenceParser;

impl TraitUsePrecedenceParser {
  pub fn test<'arena, 'a>(
    parser: &mut Parser<'arena, 'a>,
    _: &mut LoopArgument
  ) -> Option<std::vec::Vec<LookupResult<'arena>>> {
    match_pattern(
      parser,
      &[
        Lookup::Equal(&[TokenType::Identifier, TokenType::Name, TokenType::Get, TokenType::Set]),
        Lookup::Optional(&[TokenType::DoubleColon]),
        Lookup::Optional(&[TokenType::Identifier, TokenType::Name, TokenType::Get, TokenType::Set]),
        Lookup::Equal(&[TokenType::InsteadOf]),
        Lookup::Equal(&[TokenType::Identifier, TokenType::Name, TokenType::Get, TokenType::Set]),
      ]
    )
  }

  pub fn parse<'arena, 'a, 'b>(
    parser: &mut Parser<'arena, 'a>,
    matched: std::vec::Vec<LookupResult<'arena>>,
    start_loc: Location,
    _: &mut LoopArgument<'arena, 'b>
  ) -> Result<Node<'arena>, ParserError> {
    if let [trait_name, _, method, _, instead] = matched.as_slice() {
      let instead = IdentifierParser::from_token(instead.as_equal(parser)?);
      let mut trait_name_parsed = Some(IdentifierParser::from_token(trait_name.as_equal(parser)?));

      let method = if let Some(method) = &method.as_optional(parser) {
        IdentifierParser::from_token(method)
      } else {
        let t = trait_name_parsed.unwrap();
        trait_name_parsed = None;
        t
      };
      return Ok(
        TraitUsePrecedenceNode::loc(
          trait_name_parsed.into_boxed(parser.arena),
          method.into_boxed(parser.arena),
          instead.into_boxed(parser.arena),
          parser.gen_loc(start_loc)
        )
      );
    }
    Err(ParserError::Internal)
  }
}
