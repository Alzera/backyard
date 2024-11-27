use backyard_lexer::token::{ Token, TokenType };
use backyard_nodes::node::{ Location, Node, UseItemNode, UseNode };

use crate::{
  error::ParserError,
  guard,
  parser::{ LoopArgument, Parser },
  utils::{ match_pattern, some_or_default, Lookup },
};

use super::{ comment::CommentParser, identifier::IdentifierParser };

#[derive(Debug, Clone)]
pub struct UseParser;

impl UseParser {
  pub fn test(tokens: &[Token], _: &mut LoopArgument) -> Option<Vec<Vec<Token>>> {
    match_pattern(tokens, &[Lookup::Equal(&[TokenType::Use])])
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<Vec<Token>>,
    start_loc: Location,
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
        let name = guard!(parser.tokens.get(parser.position), {
          return Err(ParserError::internal("Use", args));
        }).value.to_owned();
        parser.position += 1;

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
        return Ok(UseNode::new(Some(name), items, parser.gen_loc(start_loc)));
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
        return Ok(UseNode::new(None, items, parser.gen_loc(start_loc)));
      }
    }
    Err(ParserError::internal("Use", args))
  }
}

#[derive(Debug, Clone)]
pub struct UseItemParser;

impl UseItemParser {
  pub fn test(tokens: &[Token], _: &mut LoopArgument) -> Option<Vec<Vec<Token>>> {
    match_pattern(
      tokens,
      &[
        Lookup::Optional(&[TokenType::Function, TokenType::Const]),
        Lookup::Equal(&[TokenType::Identifier, TokenType::Name]),
      ]
    )
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<Vec<Token>>,
    start_loc: Location,
    args: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [modifier, name] = matched.as_slice() {
      let modifier = some_or_default(modifier.first(), String::from(""), |i| i.value.to_owned());
      let name = guard!(name.first(), {
        return Err(ParserError::internal("UseItem", args));
      }).value.to_owned();
      let mut alias = None;
      if let Some(last) = parser.tokens.get(parser.position) {
        if last.token_type == TokenType::As {
          if let Some(id) = parser.tokens.get(parser.position + 1) {
            alias = Some(IdentifierParser::from_token(id));
            parser.position += 2;
          } else {
            return Err(ParserError::internal("UseItem", args));
          }
        }
      }
      return Ok(UseItemNode::new(modifier, name, alias, parser.gen_loc(start_loc)));
    }
    Err(ParserError::internal("UseItem", args))
  }
}
