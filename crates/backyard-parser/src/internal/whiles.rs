use backyard_lexer::token::{ Token, TokenType };
use backyard_nodes::{ Location, Node, WhileNode, utils::IntoBoxedNode };

use crate::{
  error::ParserError,
  guard,
  parser::{ LoopArgument, Parser },
  utils::{ match_pattern, Lookup, LookupResult },
};

use super::block::BlockParser;

#[derive(Debug, Clone)]
pub struct WhileParser;

impl WhileParser {
  pub fn test<'arena, 'a>(
    parser: &mut Parser<'arena, 'a>,
    tokens: &[Token],
    _: &mut LoopArgument
  ) -> Option<std::vec::Vec<LookupResult<'arena>>> {
    match_pattern(
      parser,
      tokens,
      &[Lookup::Equal(&[TokenType::While]), Lookup::Equal(&[TokenType::LeftParenthesis])]
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
          &mut LoopArgument::with_tokens(parser.arena, "while", &[], &[TokenType::RightParenthesis])
        )?
      );
      parser.position += 1;
      let (is_short, body) = BlockParser::new_or_short_or_single(
        parser,
        &[TokenType::EndWhile],
        args
      )?;
      return Ok(
        WhileNode::loc(
          condition.into_boxed(parser.arena),
          body.into_boxed(parser.arena),
          is_short,
          parser.gen_loc(start_loc)
        )
      );
    }
    Err(ParserError::Internal)
  }
}
