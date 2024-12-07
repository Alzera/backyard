use backyard_lexer::token::{ Token, TokenType };
use backyard_nodes::node::{
  Location,
  Node,
  TraitUseAliasNode,
  TraitUseNode,
  TraitUsePrecedenceNode,
  Visibility,
};

use crate::{
  error::ParserError,
  guard,
  parser::{ LoopArgument, Parser },
  utils::{ match_pattern, Lookup, LookupResult, LookupResultWrapper },
};

use super::{ comment::CommentParser, identifier::IdentifierParser };

#[derive(Debug, Clone)]
pub struct TraitUseParser;

impl TraitUseParser {
  pub fn test(tokens: &[Token], _: &mut LoopArgument) -> Option<Vec<LookupResult>> {
    match_pattern(tokens, &[Lookup::Equal(&[TokenType::Use])])
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<LookupResult>,
    start_loc: Location,
    _: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [_] = matched.as_slice() {
      let traits = parser.get_children(
        &mut LoopArgument::new(
          "traituse",
          &[TokenType::Comma],
          &[TokenType::Semicolon, TokenType::LeftCurlyBracket],
          &[
            (IdentifierParser::test, IdentifierParser::parse),
            (CommentParser::test, CommentParser::parse),
          ]
        )
      )?;
      let mut adaptations = vec![];
      if guard!(parser.tokens.get(parser.position - 1)).token_type == TokenType::Semicolon {
        parser.position -= 1;
      } else {
        adaptations = parser.get_children(
          &mut LoopArgument::new(
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
  pub fn test(tokens: &[Token], _: &mut LoopArgument) -> Option<Vec<LookupResult>> {
    match_pattern(
      tokens,
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

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<LookupResult>,
    start_loc: Location,
    _: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [trait_name, double_colon, name, _, visibility, alias] = matched.as_slice() {
      let trait_name = IdentifierParser::from_token(trait_name.as_equal()?);
      let name = name.as_optional().map(IdentifierParser::from_token);
      let (trait_name_parsed, name_parsed) = if !double_colon.is_empty() {
        (Some(trait_name), name.unwrap())
      } else {
        (None, trait_name)
      };
      let alias = alias.as_optional().map(IdentifierParser::from_token);
      let visibility = visibility
        .as_optional()
        .map(|x| x.value.to_owned())
        .unwrap_or_default();
      return Ok(
        TraitUseAliasNode::loc(
          trait_name_parsed,
          name_parsed,
          alias,
          Visibility::try_from(visibility.as_str()).ok(),
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
  pub fn test(tokens: &[Token], _: &mut LoopArgument) -> Option<Vec<LookupResult>> {
    match_pattern(
      tokens,
      &[
        Lookup::Equal(&[TokenType::Identifier, TokenType::Name, TokenType::Get, TokenType::Set]),
        Lookup::Optional(&[TokenType::DoubleColon]),
        Lookup::Optional(&[TokenType::Identifier, TokenType::Name, TokenType::Get, TokenType::Set]),
        Lookup::Equal(&[TokenType::InsteadOf]),
        Lookup::Equal(&[TokenType::Identifier, TokenType::Name, TokenType::Get, TokenType::Set]),
      ]
    )
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<LookupResult>,
    start_loc: Location,
    _: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [trait_name, _, method, _, instead] = matched.as_slice() {
      let instead = IdentifierParser::from_token(instead.as_equal()?);
      let mut trait_name_parsed = Some(IdentifierParser::from_token(trait_name.as_equal()?));
      let method = if let LookupResultWrapper::Optional(Some(method)) = &method.wrapper {
        IdentifierParser::from_token(method)
      } else {
        let t = trait_name_parsed.to_owned().unwrap();
        trait_name_parsed = None;
        t
      };
      return Ok(
        TraitUsePrecedenceNode::loc(trait_name_parsed, method, instead, parser.gen_loc(start_loc))
      );
    }
    Err(ParserError::Internal)
  }
}
