use backyard_lexer::token::{ Token, TokenType };
use backyard_nodes::{
  node::{ CatchNode, FinallyNode, Location, Node, TryNode },
  utils::{ IntoBoxedNode, IntoBoxedOptionNode },
};

use crate::{
  error::ParserError,
  parser::{ LoopArgument, Parser },
  utils::{ match_pattern, Lookup, LookupResult },
};

use super::{
  block::BlockParser,
  comment::CommentParser,
  identifier::IdentifierParser,
  variable::VariableParser,
};

#[derive(Debug, Clone)]
pub struct TryParser;

impl TryParser {
  pub fn test<'arena, 'a>(
    parser: &mut Parser<'arena, 'a>,
    tokens: &[Token],
    _: &mut LoopArgument
  ) -> Option<std::vec::Vec<LookupResult<'arena>>> {
    match_pattern(parser, tokens, &[Lookup::Equal(&[TokenType::Try])])
  }

  pub fn parse<'arena, 'a, 'b>(
    parser: &mut Parser<'arena, 'a>,
    matched: std::vec::Vec<LookupResult>,
    start_loc: Location,
    _: &mut LoopArgument<'arena, 'b>
  ) -> Result<Node<'arena>, ParserError> {
    if let [_] = matched.as_slice() {
      let body = BlockParser::new_block(parser)?;
      let catches = parser.get_children(
        &mut LoopArgument::safe(
          parser.arena,
          "try",
          &[],
          &[],
          &[
            (FinallyParser::test, FinallyParser::parse),
            (CatchParser::test, CatchParser::parse),
            (CommentParser::test, CommentParser::parse),
          ]
        )
      )?;
      return Ok(TryNode::loc(body.into_boxed(parser.arena), catches, parser.gen_loc(start_loc)));
    }
    Err(ParserError::Internal)
  }
}

#[derive(Debug, Clone)]
pub struct CatchParser;

impl CatchParser {
  pub fn test<'arena, 'a>(
    parser: &mut Parser<'arena, 'a>,
    tokens: &[Token],
    _: &mut LoopArgument
  ) -> Option<std::vec::Vec<LookupResult<'arena>>> {
    match_pattern(
      parser,
      tokens,
      &[Lookup::Equal(&[TokenType::Catch]), Lookup::Equal(&[TokenType::LeftParenthesis])]
    )
  }

  pub fn parse<'arena, 'a, 'b>(
    parser: &mut Parser<'arena, 'a>,
    matched: std::vec::Vec<LookupResult>,
    start_loc: Location,
    _: &mut LoopArgument<'arena, 'b>
  ) -> Result<Node<'arena>, ParserError> {
    if let [_, _] = matched.as_slice() {
      let types = parser.get_children(
        &mut LoopArgument::new(
          parser.arena,
          "catch_types",
          &[TokenType::BitwiseOr],
          &[TokenType::Variable, TokenType::VariableBracketOpen, TokenType::RightParenthesis],
          &[
            (IdentifierParser::test, IdentifierParser::parse),
            (CommentParser::test, CommentParser::parse),
          ]
        )
      )?;
      parser.position -= 1;
      let mut variable = None;
      if let Some(last_token) = parser.tokens.get(parser.position) {
        if last_token.token_type != TokenType::RightParenthesis {
          variable = parser.get_statement(
            &mut LoopArgument::new(
              parser.arena,
              "catch_variable",
              &[],
              &[TokenType::RightParenthesis],
              &[
                (VariableParser::test, VariableParser::parse),
                (CommentParser::test, CommentParser::parse),
              ]
            )
          )?;
        }
      }
      parser.position += 1;
      let body = BlockParser::new_block(parser)?;
      return Ok(
        CatchNode::loc(
          types,
          variable.into_boxed(parser.arena),
          body.into_boxed(parser.arena),
          parser.gen_loc(start_loc)
        )
      );
    }
    Err(ParserError::Internal)
  }
}

#[derive(Debug, Clone)]
pub struct FinallyParser;

impl FinallyParser {
  pub fn test<'arena, 'a>(
    parser: &mut Parser<'arena, 'a>,
    tokens: &[Token],
    _: &mut LoopArgument
  ) -> Option<std::vec::Vec<LookupResult<'arena>>> {
    match_pattern(parser, tokens, &[Lookup::Equal(&[TokenType::Finally])])
  }

  pub fn parse<'arena, 'a, 'b>(
    parser: &mut Parser<'arena, 'a>,
    matched: std::vec::Vec<LookupResult>,
    start_loc: Location,
    _: &mut LoopArgument<'arena, 'b>
  ) -> Result<Node<'arena>, ParserError> {
    if let [_] = matched.as_slice() {
      let body = BlockParser::new_block(parser)?;
      return Ok(FinallyNode::loc(body.into_boxed(parser.arena), parser.gen_loc(start_loc)));
    }
    Err(ParserError::Internal)
  }
}
