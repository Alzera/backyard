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
    match_pattern(
      tokens,
      [
        Lookup::Optional(vec![TokenType::Abstract, TokenType::Final]),
        Lookup::Equal(vec![TokenType::Class]),
        Lookup::Optional(vec![TokenType::Identifier]),
        Lookup::Optional(vec![TokenType::Extends]),
        Lookup::Optional(vec![TokenType::Identifier]),
        Lookup::Optional(vec![TokenType::Implements]),
      ].to_vec()
    )
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<Vec<Token>>,
    args: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [modifier, _, name, _, extends, _] = matched.as_slice() {
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
          BlockNode::new(body)
        )
      );
    }
    Err(ParserError::internal("Class", args))
  }
}
