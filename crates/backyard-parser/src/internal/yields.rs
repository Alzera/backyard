use backyard_lexer::token::TokenType;
use backyard_nodes::{
  Location,
  Node,
  YieldFromNode,
  YieldNode,
  utils::{ IntoBoxedNode, IntoBoxedOptionNode },
};

use crate::{
  error::ParserError,
  parser::{ LoopArgument, OptionNodeOrInternal, Parser, TokenTypeArrayCombine },
  utils::{ match_pattern, Lookup, LookupResult },
};

#[derive(Debug, Clone)]
pub struct YieldParser;

impl YieldParser {
  pub fn test<'arena, 'a>(
    parser: &mut Parser<'arena, 'a>,
    _: &mut LoopArgument
  ) -> Option<std::vec::Vec<LookupResult<'arena>>> {
    match_pattern(
      parser,
      &[Lookup::Equal(&[TokenType::Yield]), Lookup::Optional(&[TokenType::From])]
    )
  }

  pub fn parse<'arena, 'a, 'b>(
    parser: &mut Parser<'arena, 'a>,
    matched: std::vec::Vec<LookupResult<'arena>>,
    start_loc: Location,
    args: &mut LoopArgument<'arena, 'b>
  ) -> Result<Node<'arena>, ParserError> {
    if let [_, has_from] = matched.as_slice() {
      if !has_from.is_empty() {
        let expr = parser
          .get_statement(
            &mut LoopArgument::with_tokens(
              parser.arena,
              "yield_from",
              &[],
              &args.breakers.combine(args.separators)
            )
          )?
          .ok_internal()?;
        return Ok(YieldFromNode::loc(expr.into_boxed(parser.arena), parser.gen_loc(start_loc)));
      }
      let mut value = parser.get_statement(
        &mut LoopArgument::with_tokens(
          parser.arena,
          "yield",
          &[],
          &args.breakers.combine(args.separators).combine(&[TokenType::Arrow])
        )
      )?;
      if value.is_none() {
        return Ok(YieldNode::loc(None, None, parser.gen_loc(start_loc)));
      }
      let mut key = None;
      if parser.get_token(parser.position)?.token_type == TokenType::Arrow {
        key = Some(value.unwrap());
        parser.position += 1;
        value = Some(
          parser
            .get_statement(
              &mut LoopArgument::with_tokens(
                parser.arena,
                "singles",
                &args.separators.combine(&[]),
                &args.breakers.combine(&[TokenType::Semicolon])
              )
            )?
            .ok_internal()?
        );
      }
      return Ok(
        YieldNode::loc(
          key.into_boxed(parser.arena),
          value.into_boxed(parser.arena),
          parser.gen_loc(start_loc)
        )
      );
    }
    Err(ParserError::Internal)
  }
}
