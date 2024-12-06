use backyard_lexer::token::{ Token, TokenType };
use backyard_nodes::node::{ BlockNode, CaseNode, Location, Node, SwitchNode };

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
  pub fn test(tokens: &[Token], _: &mut LoopArgument) -> Option<Vec<LookupResult>> {
    match_pattern(
      tokens,
      &[Lookup::Equal(&[TokenType::Switch]), Lookup::Equal(&[TokenType::LeftParenthesis])]
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
          &mut LoopArgument::with_tokens("switch", &[], &[TokenType::RightParenthesis])
        )?
      );
      parser.position += 1;
      let is_short = guard!(parser.tokens.get(parser.position)).token_type == TokenType::Colon;
      let block_loc = parser.tokens.get(parser.position).unwrap().get_location().unwrap();
      parser.position += 1;
      let statements = parser.get_children(
        &mut LoopArgument::new(
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
          condition,
          BlockNode::loc(statements, parser.gen_loc(block_loc)),
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
  pub fn test(tokens: &[Token], _: &mut LoopArgument) -> Option<Vec<LookupResult>> {
    match_pattern(tokens, &[Lookup::Equal(&[TokenType::Case, TokenType::Default])])
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<LookupResult>,
    start_loc: Location,
    _: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [is_default] = matched.as_slice() {
      let condition = if let LookupResultWrapper::Equal(is_default) = &is_default.wrapper {
        if is_default.token_type == TokenType::Default {
          None
        } else {
          parser.get_statement(
            &mut LoopArgument::with_tokens("switch_case_condition", &[], &[TokenType::Colon])
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
      return Ok(CaseNode::loc(condition, statements, parser.gen_loc(start_loc)));
    }
    Err(ParserError::Internal)
  }
}
