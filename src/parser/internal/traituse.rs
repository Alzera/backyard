use crate::{
  guard,
  lexer::token::{ Token, TokenType },
  parser::{
    node::{ BlockNode, Node, TraitUseAliasNode, TraitUseNode, TraitUsePrecedenceNode },
    parser::{ Internal, LoopArgument, Parser, ParserInternal },
    utils::{ match_pattern, some_or_default, Lookup },
  },
};

use super::identifier::IdentifierParser;

#[derive(Debug, Clone)]
pub struct TraitUseParser {}

impl Internal for TraitUseParser {
  fn test(&self, tokens: &Vec<Token>, _: &LoopArgument) -> Option<Vec<Vec<Token>>> {
    match_pattern(tokens, [Lookup::Equal(vec![TokenType::Use])].to_vec())
  }

  fn parse(&self, parser: &mut Parser, matched: Vec<Vec<Token>>, _: &LoopArgument) -> Option<Node> {
    if let [_] = matched.as_slice() {
      let traits = parser.get_children(
        &mut LoopArgument::new(
          "traituse",
          &[TokenType::Comma],
          &[TokenType::Semicolon, TokenType::LeftCurlyBracket],
          &[ParserInternal::Identifier(IdentifierParser {})]
        )
      );
      let mut adaptations: Option<Node> = None;
      if guard!(parser.tokens.get(parser.position - 1)).token_type == TokenType::Semicolon {
        parser.position -= 1;
      } else {
        let adaptations_body = parser.get_children(
          &mut LoopArgument::new(
            "traituse_body",
            &[TokenType::Semicolon],
            &[TokenType::RightCurlyBracket],
            &[
              ParserInternal::TraitUseAlias(TraitUseAliasParser {}),
              ParserInternal::TraitUsePrecedence(TraitUsePrecedenceParser {}),
            ]
          )
        );
        adaptations = Some(Box::new(BlockNode { statements: adaptations_body }));
      }
      return Some(
        Box::new(TraitUseNode {
          traits,
          adaptations,
        })
      );
    }
    None
  }
}

#[derive(Debug, Clone)]
pub struct TraitUseAliasParser {}

impl Internal for TraitUseAliasParser {
  fn test(&self, tokens: &Vec<Token>, _: &LoopArgument) -> Option<Vec<Vec<Token>>> {
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

  fn parse(&self, _: &mut Parser, matched: Vec<Vec<Token>>, _: &LoopArgument) -> Option<Node> {
    if let [trait_name, double_colon, name, _, visibility, alias] = matched.as_slice() {
      let has_trait = double_colon.len() > 0;
      let trait_to_parsed = if has_trait { trait_name } else { name };
      let name_to_parsed = if has_trait { name } else { trait_name };
      let trait_name_parsed = match trait_to_parsed.get(0) {
        Some(t) => Some(IdentifierParser::new(t.value.to_owned())),
        _ => None,
      };
      return Some(
        Box::new(TraitUseAliasNode {
          trait_name: trait_name_parsed,
          method: IdentifierParser::from_matched(name_to_parsed),
          alias: IdentifierParser::from_matched(alias),
          visibility: some_or_default(visibility.get(0), String::from(""), |i| i.value.to_owned()),
        })
      );
    }
    None
  }
}

#[derive(Debug, Clone)]
pub struct TraitUsePrecedenceParser {}

impl Internal for TraitUsePrecedenceParser {
  fn test(&self, tokens: &Vec<Token>, _: &LoopArgument) -> Option<Vec<Vec<Token>>> {
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

  fn parse(&self, _: &mut Parser, matched: Vec<Vec<Token>>, _: &LoopArgument) -> Option<Node> {
    if let [trait_name, double_colon, name, _, instead] = matched.as_slice() {
      let has_trait = double_colon.len() > 0;
      let trait_to_parsed = if has_trait { trait_name } else { name };
      let name_to_parsed = if has_trait { name } else { trait_name };
      let trait_name_parsed = match trait_to_parsed.get(0) {
        Some(t) => Some(IdentifierParser::new(t.value.to_owned())),
        _ => None,
      };
      return Some(
        Box::new(TraitUsePrecedenceNode {
          trait_name: trait_name_parsed,
          method: IdentifierParser::from_matched(name_to_parsed),
          instead: IdentifierParser::from_matched(instead),
        })
      );
    }
    None
  }
}
