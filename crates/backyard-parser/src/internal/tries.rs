use backyard_lexer::token::{ Token, TokenType };
use backyard_nodes::node::{ CatchNode, FinallyNode, Location, Node, TryNode };

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
  pub fn test(tokens: &[Token], _: &mut LoopArgument) -> Option<Vec<LookupResult>> {
    match_pattern(tokens, &[Lookup::Equal(&[TokenType::Try])])
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<LookupResult>,
    start_loc: Location,
    _: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [_] = matched.as_slice() {
      let body = BlockParser::new_block(parser)?;
      let catches = parser.get_children(
        &mut LoopArgument::safe(
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
      return Ok(TryNode::loc(body, catches, parser.gen_loc(start_loc)));
    }
    Err(ParserError::Internal)
  }
}

#[derive(Debug, Clone)]
pub struct CatchParser;

impl CatchParser {
  pub fn test(tokens: &[Token], _: &mut LoopArgument) -> Option<Vec<LookupResult>> {
    match_pattern(
      tokens,
      &[Lookup::Equal(&[TokenType::Catch]), Lookup::Equal(&[TokenType::LeftParenthesis])]
    )
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<LookupResult>,
    start_loc: Location,
    _: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [_, _] = matched.as_slice() {
      let types = parser.get_children(
        &mut LoopArgument::new(
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
      return Ok(CatchNode::loc(types, variable, body, parser.gen_loc(start_loc)));
    }
    Err(ParserError::Internal)
  }
}

#[derive(Debug, Clone)]
pub struct FinallyParser;

impl FinallyParser {
  pub fn test(tokens: &[Token], _: &mut LoopArgument) -> Option<Vec<LookupResult>> {
    match_pattern(tokens, &[Lookup::Equal(&[TokenType::Finally])])
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<LookupResult>,
    start_loc: Location,
    _: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [_] = matched.as_slice() {
      let body = BlockParser::new_block(parser)?;
      return Ok(FinallyNode::loc(body, parser.gen_loc(start_loc)));
    }
    Err(ParserError::Internal)
  }
}
