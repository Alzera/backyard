use backyard_lexer::token::{ Token, TokenType };
use backyard_nodes::{
  node::{ ElseNode, IfNode, Location, Node },
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
pub struct IfParser;

impl IfParser {
  pub fn test<'arena, 'a>(
    parser: &mut Parser<'arena, 'a>,
    tokens: &[Token],
    _: &mut LoopArgument
  ) -> Option<std::vec::Vec<LookupResult<'arena>>> {
    match_pattern(
      parser,
      tokens,
      &[Lookup::Equal(&[TokenType::If]), Lookup::Equal(&[TokenType::LeftParenthesis])]
    )
  }

  pub fn parse<'arena, 'a, 'b>(
    parser: &mut Parser<'arena, 'a>,
    matched: std::vec::Vec<LookupResult>,
    start_loc: Location,
    args: &mut LoopArgument<'arena, 'b>
  ) -> Result<Node<'arena>, ParserError> {
    if let [_, _] = matched.as_slice() {
      let condition = guard!(
        parser.get_statement(
          &mut LoopArgument::with_tokens(parser.arena, "if", &[], &[TokenType::RightParenthesis])
        )?
      );
      parser.position += 1;
      let (is_short, valid) = BlockParser::new_or_short_or_single(
        parser,
        &[TokenType::ElseIf, TokenType::Else, TokenType::EndIf],
        args
      )?;
      if is_short {
        parser.position -= 1;
      }
      let invalid = parser.get_statement(
        &mut LoopArgument::safe(
          &parser.arena,
          "if_invalid",
          &[],
          &[TokenType::RightCurlyBracket, TokenType::EndIf],
          &[
            (ElseParser::test, ElseParser::parse),
            (CommentParser::test, CommentParser::parse),
          ]
        )
      )?;
      if is_short {
        if let Some(token) = parser.tokens.get(parser.position) {
          if [TokenType::EndIf].contains(&token.token_type) {
            parser.position += 1;
          }
        }
      }
      return Ok(
        IfNode::loc(
          condition.into_boxed(&parser.arena),
          valid.into_boxed(&parser.arena),
          invalid.into_boxed(&parser.arena),
          is_short,
          parser.gen_loc(start_loc)
        )
      );
    }
    Err(ParserError::Internal)
  }
}

#[derive(Debug, Clone)]
pub struct ElseParser;

impl ElseParser {
  pub fn test<'arena, 'a>(
    parser: &mut Parser<'arena, 'a>,
    tokens: &[Token],
    _: &mut LoopArgument
  ) -> Option<std::vec::Vec<LookupResult<'arena>>> {
    match_pattern(parser, tokens, &[Lookup::Equal(&[TokenType::Else, TokenType::ElseIf])])
  }

  pub fn parse<'arena, 'a, 'b>(
    parser: &mut Parser<'arena, 'a>,
    matched: std::vec::Vec<LookupResult>,
    start_loc: Location,
    args: &mut LoopArgument<'arena, 'b>
  ) -> Result<Node<'arena>, ParserError> {
    if let [keyword] = matched.as_slice() {
      if let Ok(keyword) = keyword.as_equal() {
        if keyword.token_type == TokenType::ElseIf {
          let token = parser.tokens.get(parser.position).unwrap();
          let loc = token.get_location().unwrap();
          parser.position += 1;
          let expr = IfParser::parse(
            parser,
            vec![
              LookupResult { size: 1, wrapper: LookupResultWrapper::Equal(keyword.to_owned()) },
              LookupResult { size: 1, wrapper: LookupResultWrapper::Equal(token.to_owned()) }
            ],
            loc,
            args
          )?;
          return Ok(
            ElseNode::loc(expr.into_boxed(&parser.arena), false, parser.gen_loc(start_loc))
          );
        }
      }
      if let Some(next_token) = parser.tokens.get(parser.position) {
        if next_token.token_type == TokenType::If {
          parser.position += 1;
          let token = parser.tokens.get(parser.position).unwrap();
          let loc = token.get_location().unwrap();
          parser.position += 1;
          let expr = IfParser::parse(
            parser,
            vec![
              LookupResult { size: 1, wrapper: LookupResultWrapper::Equal(next_token.to_owned()) },
              LookupResult { size: 1, wrapper: LookupResultWrapper::Equal(token.to_owned()) }
            ],
            loc,
            args
          )?;
          return Ok(
            ElseNode::loc(expr.into_boxed(&parser.arena), false, parser.gen_loc(start_loc))
          );
        }
      }
      let (is_short, valid) = BlockParser::new_or_short_or_single(
        parser,
        &[TokenType::ElseIf, TokenType::Else, TokenType::EndIf],
        args
      )?;
      return Ok(
        ElseNode::loc(valid.into_boxed(&parser.arena), is_short, parser.gen_loc(start_loc))
      );
    }
    Err(ParserError::Internal)
  }
}
