use backyard_lexer::token::{ Token, TokenType };
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
  guard,
  parser::{ LocationHelper, LoopArgument, Parser },
  utils::{ match_pattern, Lookup, LookupResult, LookupResultWrapper },
};

use super::{ block::BlockParser, comment::CommentParser };

#[derive(Debug, Clone)]
pub struct SwitchParser;

impl SwitchParser {
  pub fn test<'arena, 'a>(
    parser: &mut Parser<'arena, 'a>,
    tokens: &[Token],
    _: &mut LoopArgument
  ) -> Option<std::vec::Vec<LookupResult<'arena>>> {
    match_pattern(
      parser,
      tokens,
      &[Lookup::Equal(&[TokenType::Switch]), Lookup::Equal(&[TokenType::LeftParenthesis])]
    )
  }

  pub fn parse<'arena, 'a, 'b>(
    parser: &mut Parser<'arena, 'a>,
    matched: std::vec::Vec<LookupResult>,
    start_loc: Location,
    _: &mut LoopArgument<'arena, 'b>
  ) -> Result<Node<'arena>, ParserError> {
    if let [_, _] = matched.as_slice() {
      let condition = guard!(
        parser.get_statement(
          &mut LoopArgument::with_tokens(
            parser.arena,
            "switch",
            &[],
            &[TokenType::RightParenthesis]
          )
        )?
      );
      parser.position += 1;
      let is_short = guard!(parser.tokens.get(parser.position)).token_type == TokenType::Colon;
      let block_loc = parser.tokens.get(parser.position).unwrap().get_location().unwrap();
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
    tokens: &[Token],
    _: &mut LoopArgument
  ) -> Option<std::vec::Vec<LookupResult<'arena>>> {
    match_pattern(parser, tokens, &[Lookup::Equal(&[TokenType::Case, TokenType::Default])])
  }

  pub fn parse<'arena, 'a, 'b>(
    parser: &mut Parser<'arena, 'a>,
    matched: std::vec::Vec<LookupResult>,
    start_loc: Location,
    _: &mut LoopArgument<'arena, 'b>
  ) -> Result<Node<'arena>, ParserError> {
    if let [is_default] = matched.as_slice() {
      let condition = if let LookupResultWrapper::Equal(is_default) = &is_default.wrapper {
        if is_default.token_type == TokenType::Default {
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
        }
      } else {
        return Err(ParserError::Internal);
      };
      parser.position += 1;
      let statements = {
        let token = guard!(parser.tokens.get(parser.position)).token_type;
        if token == TokenType::LeftCurlyBracket {
          BlockParser::new_block(parser)?
        } else {
          let block_loc = parser.tokens.get(parser.position).unwrap().get_location().unwrap();
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
