use backyard_lexer::token::{ Token, TokenType };
use backyard_nodes::node::{ Node, UseItemNode, UseNode };

use crate::{
  error::ParserError,
  parser::{ LoopArgument, Parser },
  utils::{ match_pattern, some_or_default, Lookup },
};

use super::{ comment::CommentParser, identifier::IdentifierParser };

#[derive(Debug, Clone)]
pub struct UseParser {}

impl UseParser {
  pub fn test(tokens: &Vec<Token>, _: &mut LoopArgument) -> Option<Vec<Vec<Token>>> {
    match_pattern(tokens, [Lookup::Equal(vec![TokenType::Use])].to_vec())
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<Vec<Token>>,
    args: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [_] = matched.as_slice() {
      let mut p = parser.position;
      let mut has_bracket = false;
      while let Some(i) = parser.tokens.get(p) {
        if i.token_type == TokenType::Semicolon {
          break;
        } else if i.token_type == TokenType::LeftCurlyBracket {
          has_bracket = true;
          break;
        }
        p += 1;
      }
      if has_bracket {
        let mut name = vec![];
        loop {
          if let Some(token) = parser.tokens.get(parser.position) {
            if [TokenType::Identifier, TokenType::Name].contains(&token.token_type) {
              name.push(IdentifierParser::new(token.value.to_owned()));
              parser.position += 1;
              continue;
            }
          }
          break;
        }

        let items = {
          let mut items = vec![];
          if let Some(t) = parser.tokens.get(parser.position) {
            if t.token_type == TokenType::LeftCurlyBracket {
              parser.position += 1;
              items = parser.get_children(
                &mut LoopArgument::new(
                  "uses_items",
                  &[TokenType::Comma],
                  &[TokenType::RightCurlyBracket],
                  &[
                    (UseItemParser::test, UseItemParser::parse),
                    (CommentParser::test, CommentParser::parse),
                  ]
                )
              )?;
            }
          }
          items
        };
        return Ok(UseNode::new(Some(name), items));
      } else {
        let items = parser.get_children(
          &mut LoopArgument::new(
            "uses_items",
            &[TokenType::Comma],
            &[TokenType::Semicolon],
            &[
              (UseItemParser::test, UseItemParser::parse),
              (CommentParser::test, CommentParser::parse),
            ]
          )
        )?;
        return Ok(UseNode::new(None, items));
      }
    }
    Err(ParserError::internal("Use", args))
  }
}

#[derive(Debug, Clone)]
pub struct UseItemParser {}

impl UseItemParser {
  pub fn test(tokens: &Vec<Token>, _: &mut LoopArgument) -> Option<Vec<Vec<Token>>> {
    match_pattern(tokens, [Lookup::Optional(vec![TokenType::Function, TokenType::Const])].to_vec())
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<Vec<Token>>,
    args: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [modifier] = matched.as_slice() {
      let modifier = some_or_default(modifier.get(0), String::from(""), |i| i.value.to_owned());
      let mut name = vec![];
      loop {
        if let Some(token) = parser.tokens.get(parser.position) {
          if [TokenType::Identifier, TokenType::Name].contains(&token.token_type) {
            name.push(IdentifierParser::new(token.value.to_owned()));
            parser.position += 1;
            continue;
          }
        }
        break;
      }
      let mut alias = None;
      if let Some(last) = parser.tokens.get(parser.position) {
        if last.token_type == TokenType::As {
          if let Some(id) = parser.tokens.get(parser.position + 1) {
            alias = Some(IdentifierParser::new(id.value.to_owned()));
            parser.position += 2;
          } else {
            return Err(ParserError::internal("UseItem", args));
          }
        }
      }
      return Ok(UseItemNode::new(modifier, name, alias));
    }
    Err(ParserError::internal("UseItem", args))
  }
}
