use backyard_lexer::token::TokenType;
use backyard_nodes::{ ForeachNode, Location, Node, utils::{ IntoBoxedNode, IntoBoxedOptionNode } };

use crate::{
  error::ParserError,
  parser::{ LoopArgument, OptionNodeOrInternal, Parser },
  utils::{ match_pattern, Lookup, LookupResult },
};

use super::block::BlockParser;

#[derive(Debug, Clone)]
pub struct ForeachParser;

impl ForeachParser {
  fn get_key_value<'arena, 'a>(
    parser: &mut Parser<'arena, 'a>
  ) -> Result<(Option<Node<'arena>>, Node<'arena>), ParserError> {
    let key_or_value = parser
      .get_statement(
        &mut LoopArgument::with_tokens(
          parser.arena,
          "foreach_key_or_value",
          &[],
          &[TokenType::Arrow, TokenType::RightParenthesis]
        )
      )?
      .ok_internal()?;
    let has_key_type = parser.get_token(parser.position)?.token_type;
    parser.position += 1;
    if has_key_type == TokenType::RightParenthesis {
      return Ok((None, key_or_value));
    } else if has_key_type == TokenType::Arrow {
      let value = parser
        .get_statement(
          &mut LoopArgument::with_tokens(
            parser.arena,
            "foreach_value",
            &[],
            &[TokenType::RightParenthesis]
          )
        )?
        .ok_internal()?;
      parser.position += 1;
      return Ok((Some(key_or_value), value));
    }
    Err(ParserError::Internal)
  }
}

impl ForeachParser {
  pub fn test<'arena, 'a>(
    parser: &mut Parser<'arena, 'a>,
    _: &mut LoopArgument
  ) -> Option<std::vec::Vec<LookupResult<'arena>>> {
    match_pattern(
      parser,
      &[Lookup::Equal(&[TokenType::Foreach]), Lookup::Equal(&[TokenType::LeftParenthesis])]
    )
  }

  pub fn parse<'arena, 'a, 'b>(
    parser: &mut Parser<'arena, 'a>,
    matched: std::vec::Vec<LookupResult<'arena>>,
    start_loc: Location,
    args: &mut LoopArgument<'arena, 'b>
  ) -> Result<Node<'arena>, ParserError> {
    if let [_, _] = matched.as_slice() {
      let source = parser
        .get_statement(
          &mut LoopArgument::with_tokens(parser.arena, "foreach_source", &[], &[TokenType::As])
        )?
        .ok_internal()?;
      parser.position += 1;
      let (key, value) = ForeachParser::get_key_value(parser)?;
      let (is_short, body) = BlockParser::new_or_short_or_single(
        parser,
        &[TokenType::EndForeach],
        args
      )?;
      return Ok(
        ForeachNode::loc(
          source.into_boxed(parser.arena),
          key.into_boxed(parser.arena),
          value.into_boxed(parser.arena),
          body.into_boxed(parser.arena),
          is_short,
          parser.gen_loc(start_loc)
        )
      );
    }
    Err(ParserError::Internal)
  }
}
