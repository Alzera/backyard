use bumpalo::vec;
use backyard_lexer::token::TokenType;
use backyard_nodes::{ Location, MatchArmNode, MatchNode, Node, utils::IntoBoxedNode };

use crate::{
  error::ParserError,
  parser::{ LoopArgument, OptionNodeOrInternal, Parser },
  utils::{ match_pattern, Lookup, LookupResult },
};

use super::comment::CommentParser;

#[derive(Debug, Clone)]
pub struct MatchParser;

impl MatchParser {
  pub fn test<'arena, 'a>(
    parser: &mut Parser<'arena, 'a>,
    _: &mut LoopArgument
  ) -> Option<std::vec::Vec<LookupResult<'arena>>> {
    match_pattern(
      parser,
      &[Lookup::Equal(&[TokenType::Match]), Lookup::Equal(&[TokenType::LeftParenthesis])]
    )
  }

  pub fn parse<'arena, 'a, 'b>(
    parser: &mut Parser<'arena, 'a>,
    matched: std::vec::Vec<LookupResult<'arena>>,
    start_loc: Location,
    _: &mut LoopArgument<'arena, 'b>
  ) -> Result<Node<'arena>, ParserError> {
    if let [_, _] = matched.as_slice() {
      let condition = parser
        .get_statement(
          &mut LoopArgument::with_tokens(parser.arena, "match", &[], &[TokenType::RightParenthesis])
        )?
        .ok_internal()?;
      parser.position += 2;
      let arms = parser.get_children(
        &mut LoopArgument::new(
          parser.arena,
          "match_arm",
          &[TokenType::Comma],
          &[TokenType::RightCurlyBracket],
          &[
            (CommentParser::test, CommentParser::parse),
            (MatchArmParser::test, MatchArmParser::parse),
          ]
        )
      )?;
      return Ok(
        MatchNode::loc(condition.into_boxed(parser.arena), arms, parser.gen_loc(start_loc))
      );
    }
    Err(ParserError::Internal)
  }
}

#[derive(Debug, Clone)]
pub struct MatchArmParser;

impl MatchArmParser {
  pub fn test<'arena, 'a>(
    _: &mut Parser<'arena, 'a>,
    _: &mut LoopArgument
  ) -> Option<std::vec::Vec<LookupResult<'arena>>> {
    Some(std::vec![])
  }

  pub fn parse<'arena, 'a, 'b>(
    parser: &mut Parser<'arena, 'a>,
    _: std::vec::Vec<LookupResult>,
    start_loc: Location,
    _: &mut LoopArgument<'arena, 'b>
  ) -> Result<Node<'arena>, ParserError> {
    let conditions = match parser.get_token(parser.position)?.token_type {
      TokenType::Default => {
        parser.position += 2;
        vec![in parser.arena]
      }
      _ =>
        parser.get_children(
          &mut LoopArgument::with_tokens(
            parser.arena,
            "match_arm_condition",
            &[TokenType::Comma],
            &[TokenType::Arrow]
          )
        )?,
    };
    let body = parser
      .get_statement(
        &mut LoopArgument::with_tokens(
          parser.arena,
          "match_arm_body",
          &[],
          &[TokenType::Comma, TokenType::RightCurlyBracket]
        )
      )?
      .ok_internal()?;
    if let Ok(next_token) = parser.get_token(parser.position) {
      if next_token.token_type == TokenType::Comma {
        parser.position += 1;
      }
    }
    Ok(MatchArmNode::loc(conditions, body.into_boxed(parser.arena), parser.gen_loc(start_loc)))
  }
}
