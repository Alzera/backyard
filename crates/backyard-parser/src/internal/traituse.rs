use backyard_lexer::token::{ Token, TokenType };
use backyard_nodes::node::{ Node, TraitUseAliasNode, TraitUseNode, TraitUsePrecedenceNode };
use utils::guard_none;

use crate::{ parser::{ LoopArgument, Parser }, utils::{ match_pattern, some_or_default, Lookup } };

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
    _: &mut LoopArgument
  ) -> Option<Box<Node>> {
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
      );
      let mut adaptations = vec![];
      if guard_none!(parser.tokens.get(parser.position - 1)).token_type == TokenType::Semicolon {
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
        );
      }
      return Some(TraitUseNode::new(traits, adaptations));
    }
    None
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
        Lookup::Equal(vec![TokenType::Identifier]),
      ].to_vec()
    )
  }

  pub fn parse(
    _: &mut Parser,
    matched: Vec<Vec<Token>>,
    _: &mut LoopArgument
  ) -> Option<Box<Node>> {
    if let [trait_name, double_colon, name, _, visibility, alias] = matched.as_slice() {
      let has_trait = double_colon.len() > 0;
      let trait_to_parsed = if has_trait { trait_name } else { name };
      let name_to_parsed = if has_trait { name } else { trait_name };
      let trait_name_parsed = match trait_to_parsed.get(0) {
        Some(t) => Some(IdentifierParser::new(t.value.to_owned())),
        _ => None,
      };
      return Some(
        TraitUseAliasNode::new(
          trait_name_parsed,
          IdentifierParser::from_matched(name_to_parsed),
          IdentifierParser::from_matched(alias),
          some_or_default(visibility.get(0), String::from(""), |i| i.value.to_owned())
        )
      );
    }
    None
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
        Lookup::Equal(vec![TokenType::DoubleColon]),
        Lookup::Equal(vec![TokenType::Identifier]),
        Lookup::Equal(vec![TokenType::InsteadOf]),
        Lookup::Equal(vec![TokenType::Identifier]),
      ].to_vec()
    )
  }

  pub fn parse(
    _: &mut Parser,
    matched: Vec<Vec<Token>>,
    _: &mut LoopArgument
  ) -> Option<Box<Node>> {
    if let [trait_name, _, name, _, instead] = matched.as_slice() {
      return Some(
        TraitUsePrecedenceNode::new(
          IdentifierParser::from_matched(trait_name),
          IdentifierParser::from_matched(name),
          IdentifierParser::from_matched(instead)
        )
      );
    }
    None
  }
}
