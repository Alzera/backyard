use bumpalo::vec;
use backyard_lexer::token::{ Token, TokenType };
use backyard_nodes::{
  Location,
  Node,
  UseItemModifier,
  UseItemNode,
  UseNode,
  utils::IntoBoxedOptionNode,
};

use crate::{
  error::ParserError,
  guard,
  parser::{ LoopArgument, Parser },
  utils::{ match_pattern, Lookup, LookupResult },
};

use super::{ comment::CommentParser, identifier::IdentifierParser };

#[derive(Debug, Clone)]
pub struct UseParser;

impl UseParser {
  pub fn test<'arena, 'a>(
    parser: &mut Parser<'arena, 'a>,
    tokens: &[Token],
    _: &mut LoopArgument
  ) -> Option<std::vec::Vec<LookupResult<'arena>>> {
    match_pattern(parser, tokens, &[Lookup::Equal(&[TokenType::Use])])
  }

  pub fn parse<'arena, 'a, 'b>(
    parser: &mut Parser<'arena, 'a>,
    matched: std::vec::Vec<LookupResult>,
    start_loc: Location,
    _: &mut LoopArgument<'arena, 'b>
  ) -> Result<Node<'arena>, ParserError> {
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
        let name = guard!(parser.tokens.get(parser.position)).value.to_owned();
        parser.position += 1;

        let items = {
          let mut items = vec![in parser.arena];
          if let Some(t) = parser.tokens.get(parser.position) {
            if t.token_type == TokenType::LeftCurlyBracket {
              parser.position += 1;
              items = parser.get_children(
                &mut LoopArgument::new(
                  parser.arena,
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
        return Ok(UseNode::loc(Some(name), items, parser.gen_loc(start_loc)));
      } else {
        let items = parser.get_children(
          &mut LoopArgument::new(
            parser.arena,
            "uses_items",
            &[TokenType::Comma],
            &[TokenType::Semicolon],
            &[
              (UseItemParser::test, UseItemParser::parse),
              (CommentParser::test, CommentParser::parse),
            ]
          )
        )?;
        return Ok(UseNode::loc(None, items, parser.gen_loc(start_loc)));
      }
    }
    Err(ParserError::Internal)
  }
}

#[derive(Debug, Clone)]
pub struct UseItemParser;

impl UseItemParser {
  pub fn test<'arena, 'a>(
    parser: &mut Parser<'arena, 'a>,
    tokens: &[Token],
    _: &mut LoopArgument
  ) -> Option<std::vec::Vec<LookupResult<'arena>>> {
    match_pattern(
      parser,
      tokens,
      &[
        Lookup::Optional(&[TokenType::Function, TokenType::Const]),
        Lookup::Equal(
          &[TokenType::Identifier, TokenType::Name, TokenType::Name, TokenType::Get, TokenType::Set]
        ),
      ]
    )
  }

  pub fn parse<'arena, 'a, 'b>(
    parser: &mut Parser<'arena, 'a>,
    matched: std::vec::Vec<LookupResult>,
    start_loc: Location,
    _: &mut LoopArgument<'arena, 'b>
  ) -> Result<Node<'arena>, ParserError> {
    if let [modifier, name] = matched.as_slice() {
      let modifier = modifier.as_optional().and_then(|x| UseItemModifier::try_from(&x.value).ok());
      let name = name.as_equal()?.value.to_owned();
      let mut alias = None;
      if let Some(last) = parser.tokens.get(parser.position) {
        if last.token_type == TokenType::As {
          if let Some(id) = parser.tokens.get(parser.position + 1) {
            alias = Some(IdentifierParser::from_token(id));
            parser.position += 2;
          } else {
            return Err(ParserError::Internal);
          }
        }
      }
      return Ok(
        UseItemNode::loc(modifier, name, alias.into_boxed(parser.arena), parser.gen_loc(start_loc))
      );
    }
    Err(ParserError::Internal)
  }
}
