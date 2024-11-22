use backyard_lexer::token::{ Token, TokenType };
use backyard_nodes::node::{ Node, BlockNode, ClassNode };

use crate::{
  error::ParserError,
  parser::{ LoopArgument, Parser },
  utils::{ match_pattern, some_or_default, Lookup },
};

use super::{
  comment::CommentParser,
  consts::ConstPropertyParser,
  identifier::IdentifierParser,
  method::MethodParser,
  property::PropertyParser,
  traituse::TraitUseParser,
};

#[derive(Debug, Clone)]
pub struct ClassParser {}

impl ClassParser {
  pub fn test(tokens: &Vec<Token>, _: &mut LoopArgument) -> Option<Vec<Vec<Token>>> {
    let modifiers_rule = [
      [TokenType::Readonly].to_vec(),
      [TokenType::Abstract, TokenType::Final].to_vec(),
    ];
    let mut modifiers = vec![vec![], vec![]];
    let mut pos = 0;
    loop {
      let token = tokens.get(pos);
      pos += 1;
      if pos > 2 || token.is_none() {
        break;
      }
      let token = token.unwrap();
      for (i, modifier) in modifiers_rule.iter().enumerate() {
        if modifiers[i].len() > 0 {
          continue;
        }
        if modifier.contains(&token.token_type) {
          modifiers[i].push(token.clone());
          break;
        }
      }
    }
    let modifier_count = modifiers
      .iter()
      .map(|i| i.len())
      .sum::<usize>();
    if
      let Some(next_modifiers) = match_pattern(
        &tokens[modifier_count..].to_vec(),
        [
          Lookup::Equal(vec![TokenType::Class]),
          Lookup::Optional(vec![TokenType::Identifier]),
          Lookup::Optional(vec![TokenType::Extends]),
          Lookup::Optional(vec![TokenType::Identifier, TokenType::Name]),
          Lookup::Optional(vec![TokenType::Implements]),
        ].to_vec()
      )
    {
      modifiers.extend(next_modifiers);
      println!("Ale 1: {:?}", modifiers);
      return Some(modifiers);
    }
    None
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<Vec<Token>>,
    args: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [readonly, modifier, _, name, _, extends, _] = matched.as_slice() {
      let implements = parser.get_children(
        &mut LoopArgument::new(
          "class_implements",
          &[TokenType::Comma],
          &[TokenType::LeftCurlyBracket],
          &[
            (IdentifierParser::test, IdentifierParser::parse),
            (CommentParser::test, CommentParser::parse),
          ]
        )
      )?;
      let body = parser.get_children(
        &mut LoopArgument::new(
          "class_body",
          &[TokenType::Semicolon],
          &[TokenType::RightCurlyBracket],
          &[
            (TraitUseParser::test, TraitUseParser::parse),
            (MethodParser::test, MethodParser::parse),
            (ConstPropertyParser::test, ConstPropertyParser::parse),
            (PropertyParser::test, PropertyParser::parse),
            (CommentParser::test, CommentParser::parse),
          ]
        )
      )?;
      let extends = match extends.len() {
        1 => Some(IdentifierParser::from_matched(extends)),
        _ => None,
      };
      let name = if name.len() > 0 { Some(IdentifierParser::from_matched(name)) } else { None };
      return Ok(
        ClassNode::new(
          some_or_default(modifier.get(0), String::from(""), |i| i.value.to_owned()),
          name,
          extends,
          implements,
          BlockNode::new(body),
          readonly.len() > 0
        )
      );
    }
    Err(ParserError::internal("Class", args))
  }
}
