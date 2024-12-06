use backyard_lexer::token::{ Token, TokenType };
use backyard_nodes::node::{ DoWhileConditionNode, DoWhileNode, Location, Node };

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
  pub fn test(tokens: &[Token], _: &mut LoopArgument) -> Option<Vec<LookupResult>> {
    match_pattern(tokens, &[Lookup::Equal(&[TokenType::Do])])
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<LookupResult>,
    start_loc: Location,
    _: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [_] = matched.as_slice() {
      let body = BlockParser::new(parser)?;
      let condition = guard!(
        parser.get_statement(
          &mut LoopArgument::new(
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
      return Ok(DoWhileNode::new(condition, body, parser.gen_loc(start_loc)));
    }
    Err(ParserError::Internal)
  }
}

#[derive(Debug, Clone)]
pub struct DoWhileConditionParser;

impl DoWhileConditionParser {
  pub fn test(tokens: &[Token], _: &mut LoopArgument) -> Option<Vec<LookupResult>> {
    match_pattern(
      tokens,
      &[Lookup::Equal(&[TokenType::While]), Lookup::Equal(&[TokenType::LeftParenthesis])]
    )
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<LookupResult>,
    start_loc: Location,
    _: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [_, _] = matched.as_slice() {
      let condition = guard!(
        parser.get_statement(
          &mut LoopArgument::with_tokens("do_while_condition", &[], &[TokenType::RightParenthesis])
        )?
      );
      return Ok(DoWhileConditionNode::new(condition, parser.gen_loc(start_loc)));
    }
    Err(ParserError::Internal)
  }
}
