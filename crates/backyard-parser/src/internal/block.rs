use backyard_lexer::token::TokenType;
use backyard_nodes::{ BlockNode, Location, Node };

use crate::{
  error::ParserError,
  guard,
  parser::{ LocationHelper, LoopArgument, Parser, DEFAULT_PARSERS },
  utils::{ match_pattern, Lookup, LookupResult },
};

use super::comment::CommentParser;

#[derive(Debug, Clone)]
pub struct BlockParser;

impl BlockParser {
  pub fn new_block<'arena, 'a>(
    parser: &mut Parser<'arena, 'a>
  ) -> Result<Node<'arena>, ParserError> {
    if
      let Some(block) = parser.get_statement(
        &mut LoopArgument::safe(
          parser.arena,
          "block_parser",
          &[],
          &[],
          &[
            (CommentParser::test, CommentParser::parse),
            (BlockParser::test, BlockParser::parse),
          ]
        )
      )?
    {
      Ok(block)
    } else {
      Err(ParserError::Internal)
    }
  }

  pub fn new_short<'arena, 'a>(
    parser: &mut Parser<'arena, 'a>,
    breakers: &[TokenType]
  ) -> Result<Node<'arena>, ParserError> {
    let start_loc = parser.tokens.get(parser.position).unwrap().get_location().unwrap();
    parser.position += 1;
    Ok(
      BlockNode::loc(
        parser.get_children(
          &mut LoopArgument::with_tokens(
            parser.arena,
            "block_short",
            &[TokenType::Semicolon],
            breakers
          )
        )?,
        parser.gen_loc(start_loc)
      )
    )
  }

  pub fn new_or_short<'arena, 'a>(
    parser: &mut Parser<'arena, 'a>,
    breakers: &[TokenType],
    _: &mut LoopArgument
  ) -> Result<(bool, Node<'arena>), ParserError> {
    if let Some(start) = parser.tokens.get(parser.position) {
      return match start.token_type {
        TokenType::Colon => Ok((true, BlockParser::new_short(parser, breakers)?)),
        TokenType::LeftCurlyBracket => Ok((false, BlockParser::new_block(parser)?)),
        _ => Err(ParserError::Internal),
      };
    }
    Err(ParserError::Internal)
  }

  pub fn new_or_short_or_single<'arena, 'a>(
    parser: &mut Parser<'arena, 'a>,
    breakers: &[TokenType],
    _: &mut LoopArgument
  ) -> Result<(bool, Node<'arena>), ParserError> {
    if let Some(start) = parser.tokens.get(parser.position) {
      return match start.token_type {
        TokenType::Colon => Ok((true, BlockParser::new_short(parser, breakers)?)),
        TokenType::LeftCurlyBracket => Ok((false, BlockParser::new_block(parser)?)),
        _ => {
          let expr = guard!(
            parser.get_statement(
              &mut LoopArgument::safe(
                parser.arena,
                "block_expr",
                &[],
                &[TokenType::Semicolon],
                &DEFAULT_PARSERS
              )
            )?
          );
          if let Some(token) = parser.tokens.get(parser.position) {
            if token.token_type == TokenType::Semicolon {
              parser.position += 1;
            }
          }
          Ok((false, expr))
        }
      };
    }
    Err(ParserError::Internal)
  }

  pub fn test<'arena, 'a>(
    parser: &mut Parser<'arena, 'a>,
    _: &mut LoopArgument
  ) -> Option<std::vec::Vec<LookupResult<'arena>>> {
    match_pattern(parser, &[Lookup::Equal(&[TokenType::LeftCurlyBracket])])
  }

  pub fn parse<'arena, 'a, 'b>(
    parser: &mut Parser<'arena, 'a>,
    matched: std::vec::Vec<LookupResult<'arena>>,
    start_loc: Location,
    _: &mut LoopArgument<'arena, 'b>
  ) -> Result<Node<'arena>, ParserError> {
    if let [_] = matched.as_slice() {
      return Ok(
        BlockNode::loc(
          parser.get_children(&mut LoopArgument::default(parser.arena, "block_parser"))?,
          parser.gen_loc(start_loc)
        )
      );
    }
    Err(ParserError::Internal)
  }
}
