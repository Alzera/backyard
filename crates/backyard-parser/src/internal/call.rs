use std::vec;

use backyard_lexer::token::{ Token, TokenType };
use backyard_nodes::node::{ CallArgumentNode, CallNode, Location, Node };

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
  pub fn get_arguments(parser: &mut Parser) -> Result<Vec<Box<Node>>, ParserError> {
    parser.get_children(
      &mut LoopArgument::new(
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
  pub fn test(tokens: &[Token], args: &mut LoopArgument) -> Option<Vec<LookupResult>> {
    args.last_expr.as_ref()?;
    match_pattern(tokens, &[Lookup::Equal(&[TokenType::LeftParenthesis])])
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<LookupResult>,
    start_loc: Location,
    args: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [_] = matched.as_slice() {
      return Ok(
        CallNode::new(
          args.last_expr.to_owned().unwrap(),
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
  pub fn test(tokens: &[Token], _: &mut LoopArgument) -> Option<Vec<LookupResult>> {
    if let Some(is_colon) = tokens.get(1) {
      if is_colon.token_type == TokenType::Colon {
        if let Some(name) = tokens.first() {
          return Some(
            vec![
              LookupResult {
                size: 1,
                wrapper: LookupResultWrapper::Optional(Some(name.to_owned())),
              },
              LookupResult {
                size: 1,
                wrapper: LookupResultWrapper::Optional(Some(is_colon.to_owned())),
              }
            ]
          );
        }
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

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<LookupResult>,
    start_loc: Location,
    _: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [name, has_name] = matched.as_slice() {
      let name = if let LookupResultWrapper::Optional(_) = &has_name.wrapper {
        if let LookupResultWrapper::Optional(Some(name)) = &name.wrapper {
          Some(IdentifierParser::from_token(name))
        } else {
          None
        }
      } else {
        return Err(ParserError::Internal);
      };
      let value = guard!(
        parser.get_statement(
          &mut LoopArgument::with_tokens(
            "argument",
            &[TokenType::Comma, TokenType::RightParenthesis],
            &[]
          )
        )?
      );
      return Ok(CallArgumentNode::new(name, value, parser.gen_loc(start_loc)));
    }
    Err(ParserError::Internal)
  }
}
