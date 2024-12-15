use bumpalo::collections::Vec;
use backyard_lexer::token::TokenType;
use backyard_nodes::{
  CallArgumentNode,
  CallNode,
  Location,
  Node,
  utils::{ IntoBoxedNode, IntoBoxedOptionNode },
};

use crate::{
  error::ParserError,
  guard,
  parser::{ LoopArgument, Parser },
  utils::{ match_pattern, Lookup, LookupResult, LookupResultWrapper },
};

use super::{ comment::CommentParser, identifier::IdentifierParser };

#[derive(Debug, Clone)]
pub struct CallParser;

impl CallParser {
  pub fn get_arguments<'arena, 'a>(
    parser: &mut Parser<'arena, 'a>
  ) -> Result<Vec<'arena, Node<'arena>>, ParserError> {
    parser.get_children(
      &mut LoopArgument::new(
        parser.arena,
        "call",
        &[TokenType::Comma],
        &[TokenType::RightParenthesis],
        &[
          (CommentParser::test, CommentParser::parse),
          (ArgumentParser::test, ArgumentParser::parse),
        ]
      )
    )
  }
}

impl CallParser {
  pub fn test<'arena, 'a>(
    parser: &mut Parser<'arena, 'a>,
    args: &mut LoopArgument
  ) -> Option<std::vec::Vec<LookupResult<'arena>>> {
    args.last_expr.as_ref()?;
    match_pattern(parser, &[Lookup::Equal(&[TokenType::LeftParenthesis])])
  }

  pub fn parse<'arena, 'a, 'b>(
    parser: &mut Parser<'arena, 'a>,
    matched: std::vec::Vec<LookupResult<'arena>>,
    start_loc: Location,
    args: &mut LoopArgument<'arena, 'b>
  ) -> Result<Node<'arena>, ParserError> {
    if let [_] = matched.as_slice() {
      return Ok(
        CallNode::loc(
          args.last_expr.take().unwrap().into_boxed(parser.arena),
          CallParser::get_arguments(parser)?,
          parser.gen_loc(start_loc)
        )
      );
    }
    Err(ParserError::Internal)
  }
}

#[derive(Debug, Clone)]
pub struct ArgumentParser;

impl ArgumentParser {
  pub fn test<'arena, 'a>(
    parser: &mut Parser<'arena, 'a>,
    _: &mut LoopArgument
  ) -> Option<std::vec::Vec<LookupResult<'arena>>> {
    if let Some(is_colon) = parser.tokens.get(parser.position + 1) {
      if is_colon.token_type == TokenType::Colon && parser.tokens.get(parser.position).is_some() {
        return Some(
          vec![
            LookupResult {
              size: 1,
              wrapper: LookupResultWrapper::Optional(Some(parser.position)),
            },
            LookupResult {
              size: 1,
              wrapper: LookupResultWrapper::Optional(Some(parser.position + 1)),
            }
          ]
        );
      }
    }
    Some(
      vec![
        LookupResult {
          size: 0,
          wrapper: LookupResultWrapper::Optional(None),
        },
        LookupResult {
          size: 0,
          wrapper: LookupResultWrapper::Optional(None),
        }
      ]
    )
  }

  pub fn parse<'arena, 'a, 'b>(
    parser: &mut Parser<'arena, 'a>,
    matched: std::vec::Vec<LookupResult<'arena>>,
    start_loc: Location,
    _: &mut LoopArgument<'arena, 'b>
  ) -> Result<Node<'arena>, ParserError> {
    if let [name, has_name] = matched.as_slice() {
      let name = has_name
        .as_optional(parser)
        .map(|_| name.as_optional(parser).map(IdentifierParser::from_token))
        .unwrap_or_default();
      let value = guard!(
        parser.get_statement(
          &mut LoopArgument::with_tokens(
            parser.arena,
            "argument",
            &[TokenType::Comma, TokenType::RightParenthesis],
            &[]
          )
        )?
      );
      return Ok(
        CallArgumentNode::loc(
          name.into_boxed(parser.arena),
          value.into_boxed(parser.arena),
          parser.gen_loc(start_loc)
        )
      );
    }
    Err(ParserError::Internal)
  }
}
