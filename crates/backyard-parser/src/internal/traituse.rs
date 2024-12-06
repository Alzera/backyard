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
      return Ok(TraitUseNode::new(traits, adaptations, parser.gen_loc(start_loc)));
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
      let has_trait = !double_colon.is_empty();
      let trait_name = if let LookupResultWrapper::Equal(trait_name) = &trait_name.wrapper {
        IdentifierParser::from_token(trait_name)
      } else {
        return Err(ParserError::Internal);
      };
      let name = if let LookupResultWrapper::Optional(name) = &name.wrapper {
        name.to_owned().map(|x| IdentifierParser::from_token(&x))
      } else {
        return Err(ParserError::Internal);
      };
      let mut trait_name_parsed = None;
      let name_parsed = if has_trait {
        trait_name_parsed = Some(trait_name);
        name.unwrap()
      } else {
        trait_name
      };
      let alias = if let LookupResultWrapper::Optional(Some(alias)) = &alias.wrapper {
        Some(IdentifierParser::from_token(alias))
      } else {
        None
      };
      let visibility = if let LookupResultWrapper::Optional(visibility) = &visibility.wrapper {
        visibility
          .as_ref()
          .map(|i| i.value.to_owned())
          .unwrap_or_default()
      } else {
        "".to_owned()
      };
      return Ok(
        TraitUseAliasNode::new(
          trait_name_parsed,
          name_parsed,
          alias,
          Visibility::try_parse(&visibility),
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
      let instead = if let LookupResultWrapper::Equal(instead) = &instead.wrapper {
        IdentifierParser::from_token(instead)
      } else {
        return Err(ParserError::Internal);
      };
      let mut trait_name_parsed = Some(
        IdentifierParser::from_token(
          if let LookupResultWrapper::Equal(trait_name) = &trait_name.wrapper {
            trait_name
          } else {
            return Err(ParserError::Internal);
          }
        )
      );
      let method = if let LookupResultWrapper::Optional(Some(method)) = &method.wrapper {
        IdentifierParser::from_token(method)
      } else {
        let t = trait_name_parsed.to_owned().unwrap();
        trait_name_parsed = None;
        t
      };
      return Ok(
        TraitUsePrecedenceNode::new(trait_name_parsed, method, instead, parser.gen_loc(start_loc))
      );
    }
    Err(ParserError::Internal)
  }
}
