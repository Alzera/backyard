use backyard_lexer::token::TokenType;
use backyard_nodes::{ BodyType, ForNode, Location, Node, utils::IntoBoxedOptionNode };

use crate::{
  error::ParserError,
  parser::{ LoopArgument, Parser },
  utils::{ match_pattern, Lookup, LookupResult },
};

use super::block::BlockParser;

#[derive(Debug, Clone)]
pub struct ForParser;

impl ForParser {
  pub fn test<'arena, 'a>(
    parser: &mut Parser<'arena, 'a>,
    _: &mut LoopArgument
  ) -> Option<std::vec::Vec<LookupResult<'arena>>> {
    match_pattern(
      parser,
      &[Lookup::Equal(&[TokenType::For]), Lookup::Equal(&[TokenType::LeftParenthesis])]
    )
  }

  pub fn parse<'arena, 'a, 'b>(
    parser: &mut Parser<'arena, 'a>,
    matched: std::vec::Vec<LookupResult<'arena>>,
    start_loc: Location,
    args: &mut LoopArgument<'arena, 'b>
  ) -> Result<Node<'arena>, ParserError> {
    if let [_, _] = matched.as_slice() {
      let inits = parser.get_children(
        &mut LoopArgument::with_tokens(
          parser.arena,
          "for_inits",
          &[TokenType::Comma],
          &[TokenType::Semicolon]
        )
      )?;
      let tests = parser.get_children(
        &mut LoopArgument::with_tokens(
          parser.arena,
          "for_tests",
          &[TokenType::Comma],
          &[TokenType::Semicolon]
        )
      )?;
      let increments = parser.get_children(
        &mut LoopArgument::with_tokens(
          parser.arena,
          "for_increments",
          &[TokenType::Comma],
          &[TokenType::RightParenthesis]
        )
      )?;
      let parsed_block = BlockParser::new_or_short(parser, &[TokenType::EndFor], args);
      let mut body = None;
      let mut body_type = BodyType::Empty;
      if parsed_block.is_ok() {
        let (is_short, parsed_block) = parsed_block.unwrap();
        body_type = match is_short {
          true => BodyType::Short,
          false => BodyType::Basic,
        };
        body = Some(parsed_block);
      }
      return Ok(
        ForNode::loc(
          inits,
          tests,
          increments,
          body.into_boxed(parser.arena),
          body_type,
          parser.gen_loc(start_loc)
        )
      );
    }
    Err(ParserError::Internal)
  }
}
