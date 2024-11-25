use backyard_lexer::token::{ Token, TokenType };
use backyard_nodes::node::{ Node, TraitUseAliasNode, TraitUseNode, TraitUsePrecedenceNode };
use utils::guard;

use crate::{
  error::ParserError,
  parser::{ LoopArgument, Parser },
  utils::{ match_pattern, some_or_default, Lookup },
};

use super::{ comment::CommentParser, identifier::IdentifierParser };

#[derive(Debug, Clone)]
pub struct TraitUseParser {}

impl TraitUseParser {
  pub fn test(tokens: &Vec<Token>, _: &mut LoopArgument) -> Option<Vec<Vec<Token>>> {
    match_pattern(tokens, [Lookup::Equal(vec![TokenType::Use])].to_vec())
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<Vec<Token>>,
    args: &mut LoopArgument
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
      if
        guard!(parser.tokens.get(parser.position - 1), {
          return Err(ParserError::internal("TraitUse", args));
        }).token_type == TokenType::Semicolon
      {
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
      return Ok(TraitUseNode::new(traits, adaptations));
    }
    Err(ParserError::internal("TraitUse", args))
  }
}

#[derive(Debug, Clone)]
pub struct TraitUseAliasParser {}

impl TraitUseAliasParser {
  pub fn test(tokens: &Vec<Token>, _: &mut LoopArgument) -> Option<Vec<Vec<Token>>> {
    match_pattern(
      tokens,
      [
        Lookup::Equal(vec![TokenType::Identifier]),
        Lookup::Optional(vec![TokenType::DoubleColon]),
        Lookup::Optional(vec![TokenType::Identifier]),
        Lookup::Equal(vec![TokenType::As]),
        Lookup::Optional(vec![TokenType::Public, TokenType::Private, TokenType::Protected]),
        Lookup::Optional(vec![TokenType::Identifier]),
      ].to_vec()
    )
  }

  pub fn parse(
    _: &mut Parser,
    matched: Vec<Vec<Token>>,
    args: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [trait_name, double_colon, name, _, visibility, alias] = matched.as_slice() {
      let has_trait = double_colon.len() > 0;
      let trait_to_parsed = if has_trait { trait_name } else { name };
      let name_to_parsed = if has_trait { name } else { trait_name };
      let trait_name_parsed = match trait_to_parsed.get(0) {
        Some(t) => Some(IdentifierParser::new(t.value.to_owned())),
        _ => None,
      };
      let alias = match alias.get(0) {
        Some(t) => Some(IdentifierParser::new(t.value.to_owned())),
        _ => None,
      };
      return Ok(
        TraitUseAliasNode::new(
          trait_name_parsed,
          IdentifierParser::from_matched(name_to_parsed),
          alias,
          some_or_default(visibility.get(0), String::from(""), |i| i.value.to_owned())
        )
      );
    }
    Err(ParserError::internal("TraitUseAlias", args))
  }
}

#[derive(Debug, Clone)]
pub struct TraitUsePrecedenceParser {}

impl TraitUsePrecedenceParser {
  pub fn test(tokens: &Vec<Token>, _: &mut LoopArgument) -> Option<Vec<Vec<Token>>> {
    match_pattern(
      tokens,
      [
        Lookup::Equal(vec![TokenType::Identifier]),
        Lookup::Optional(vec![TokenType::DoubleColon]),
        Lookup::Optional(vec![TokenType::Identifier]),
        Lookup::Equal(vec![TokenType::InsteadOf]),
        Lookup::Equal(vec![TokenType::Identifier]),
      ].to_vec()
    )
  }

  pub fn parse(
    _: &mut Parser,
    matched: Vec<Vec<Token>>,
    args: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [trait_name, _, method, _, instead] = matched.as_slice() {
      let mut trait_name_parsed = Some(
        IdentifierParser::new(
          guard!(trait_name.get(0), {
            return Err(ParserError::internal("TraitUsePrecedence", args));
          }).value.to_owned()
        )
      );
      let method = match method.get(0) {
        Some(t) => IdentifierParser::new(t.value.to_owned()),
        _ => {
          let t = trait_name_parsed.to_owned().unwrap();
          trait_name_parsed = None;
          t
        }
      };
      return Ok(
        TraitUsePrecedenceNode::new(
          trait_name_parsed,
          method,
          IdentifierParser::from_matched(instead)
        )
      );
    }
    Err(ParserError::internal("TraitUsePrecedence", args))
  }
}
