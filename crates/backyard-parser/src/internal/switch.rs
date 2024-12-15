use backyard_lexer::token::TokenType;
use backyard_nodes::{
  BlockNode,
  CaseNode,
  Location,
  Node,
  SwitchNode,
  utils::{ IntoBoxedNode, IntoBoxedOptionNode },
};

use crate::{
  error::ParserError,
  parser::{ LocationHelper, LoopArgument, OptionNodeOrInternal, Parser },
  utils::{ match_pattern, Lookup, LookupResult },
};

use super::{ block::BlockParser, comment::CommentParser };

#[derive(Debug, Clone)]
pub struct SwitchParser;

impl SwitchParser {
  pub fn test<'arena, 'a>(
    parser: &mut Parser<'arena, 'a>,
    _: &mut LoopArgument
  ) -> Option<std::vec::Vec<LookupResult<'arena>>> {
    match_pattern(
      parser,
      &[Lookup::Equal(&[TokenType::Switch]), Lookup::Equal(&[TokenType::LeftParenthesis])]
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
          &mut LoopArgument::with_tokens(
            parser.arena,
            "switch",
            &[],
            &[TokenType::RightParenthesis]
          )
        )?
        .ok_internal()?;
      parser.position += 1;
      let is_short = parser.get_token(parser.position)?.token_type == TokenType::Colon;
      let block_loc = parser.get_token(parser.position)?.get_location().unwrap();
      parser.position += 1;
      let statements = parser.get_children(
        &mut LoopArgument::new(
          parser.arena,
          "switch_body",
          &[],
          &[TokenType::RightCurlyBracket, TokenType::EndSwitch],
          &[
            (CaseParser::test, CaseParser::parse),
            (CommentParser::test, CommentParser::parse),
          ]
        )
      )?;
      return Ok(
        SwitchNode::loc(
          condition.into_boxed(parser.arena),
          BlockNode::loc(statements, parser.gen_loc(block_loc)).into_boxed(parser.arena),
          is_short,
          parser.gen_loc(start_loc)
        )
      );
    }
    Err(ParserError::Internal)
  }
}

#[derive(Debug, Clone)]
pub struct CaseParser;

impl CaseParser {
  pub fn test<'arena, 'a>(
    parser: &mut Parser<'arena, 'a>,
    _: &mut LoopArgument
  ) -> Option<std::vec::Vec<LookupResult<'arena>>> {
    match_pattern(parser, &[Lookup::Equal(&[TokenType::Case, TokenType::Default])])
  }

  pub fn parse<'arena, 'a, 'b>(
    parser: &mut Parser<'arena, 'a>,
    matched: std::vec::Vec<LookupResult<'arena>>,
    start_loc: Location,
    _: &mut LoopArgument<'arena, 'b>
  ) -> Result<Node<'arena>, ParserError> {
    if let [is_default] = matched.as_slice() {
      let condition = if is_default.as_equal(parser)?.token_type == TokenType::Default {
        None
      } else {
        parser.get_statement(
          &mut LoopArgument::with_tokens(
            parser.arena,
            "switch_case_condition",
            &[],
            &[TokenType::Colon]
          )
        )?
      };
      parser.position += 1;
      let statements = {
        if parser.get_token(parser.position)?.token_type == TokenType::LeftCurlyBracket {
          BlockParser::new_block(parser)?
        } else {
          let block_loc = parser.get_token(parser.position)?.get_location().unwrap();
          let s = parser.get_children(
            &mut LoopArgument::with_tokens(
              parser.arena,
              "switch_case_body",
              &[TokenType::Semicolon],
              &[
                TokenType::Case,
                TokenType::Default,
                TokenType::RightCurlyBracket,
                TokenType::EndSwitch,
              ]
            )
          )?;
          parser.position -= 1;
          BlockNode::loc(s, parser.gen_loc(block_loc))
        }
      };
      return Ok(
        CaseNode::loc(
          condition.into_boxed(parser.arena),
          statements.into_boxed(parser.arena),
          parser.gen_loc(start_loc)
        )
      );
    }
    Err(ParserError::Internal)
  }
}
