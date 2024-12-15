use backyard_lexer::token::TokenType;
use backyard_nodes::{ DoWhileConditionNode, DoWhileNode, Location, Node, utils::IntoBoxedNode };

use crate::{
  error::ParserError,
  guard,
  parser::{ LoopArgument, Parser },
  utils::{ match_pattern, Lookup, LookupResult },
};

use super::{ block::BlockParser, comment::CommentParser };

#[derive(Debug, Clone)]
pub struct DoWhileParser;

impl DoWhileParser {
  pub fn test<'arena, 'a>(
    parser: &mut Parser<'arena, 'a>,
    _: &mut LoopArgument
  ) -> Option<std::vec::Vec<LookupResult<'arena>>> {
    match_pattern(parser, &[Lookup::Equal(&[TokenType::Do])])
  }

  pub fn parse<'arena, 'a, 'b>(
    parser: &mut Parser<'arena, 'a>,
    matched: std::vec::Vec<LookupResult<'arena>>,
    start_loc: Location,
    _: &mut LoopArgument<'arena, 'b>
  ) -> Result<Node<'arena>, ParserError> {
    if let [_] = matched.as_slice() {
      let body = BlockParser::new_block(parser)?;
      let condition = guard!(
        parser.get_statement(
          &mut LoopArgument::new(
            parser.arena,
            "do_while",
            &[],
            &[TokenType::RightParenthesis],
            &[
              (DoWhileConditionParser::test, DoWhileConditionParser::parse),
              (CommentParser::test, CommentParser::parse),
            ]
          )
        )?
      );
      parser.position += 1;
      return Ok(
        DoWhileNode::loc(
          condition.into_boxed(parser.arena),
          body.into_boxed(parser.arena),
          parser.gen_loc(start_loc)
        )
      );
    }
    Err(ParserError::Internal)
  }
}

#[derive(Debug, Clone)]
pub struct DoWhileConditionParser;

impl DoWhileConditionParser {
  pub fn test<'arena, 'a>(
    parser: &mut Parser<'arena, 'a>,
    _: &mut LoopArgument
  ) -> Option<std::vec::Vec<LookupResult<'arena>>> {
    match_pattern(
      parser,
      &[Lookup::Equal(&[TokenType::While]), Lookup::Equal(&[TokenType::LeftParenthesis])]
    )
  }

  pub fn parse<'arena, 'a, 'b>(
    parser: &mut Parser<'arena, 'a>,
    matched: std::vec::Vec<LookupResult<'arena>>,
    start_loc: Location,
    _: &mut LoopArgument<'arena, 'b>
  ) -> Result<Node<'arena>, ParserError> {
    if let [_, _] = matched.as_slice() {
      let condition = guard!(
        parser.get_statement(
          &mut LoopArgument::with_tokens(
            parser.arena,
            "do_while_condition",
            &[],
            &[TokenType::RightParenthesis]
          )
        )?
      );
      return Ok(
        DoWhileConditionNode::loc(condition.into_boxed(parser.arena), parser.gen_loc(start_loc))
      );
    }
    Err(ParserError::Internal)
  }
}
