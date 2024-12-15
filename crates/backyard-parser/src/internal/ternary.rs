use backyard_lexer::token::TokenType;
use backyard_nodes::{ Location, Node, TernaryNode, utils::IntoBoxedNode };

use crate::{
  error::ParserError,
  parser::{ LoopArgument, OptionNodeOrInternal, Parser, TokenTypeArrayCombine, DEFAULT_PARSERS },
  utils::{ match_pattern, Lookup, LookupResult },
};

#[derive(Debug, Clone)]
pub struct TernaryParser;

impl TernaryParser {
  pub fn test<'arena, 'a>(
    parser: &mut Parser<'arena, 'a>,
    args: &mut LoopArgument
  ) -> Option<std::vec::Vec<LookupResult<'arena>>> {
    args.last_expr.as_ref()?;
    match_pattern(parser, &[Lookup::Equal(&[TokenType::QuestionMark])])
  }

  pub fn parse<'arena, 'a, 'b>(
    parser: &mut Parser<'arena, 'a>,
    matched: std::vec::Vec<LookupResult<'arena>>,
    start_loc: Location,
    args: &mut LoopArgument<'arena, 'b>
  ) -> Result<Node<'arena>, ParserError> {
    if let [_] = matched.as_slice() {
      let left = args.last_expr.take().unwrap();
      args.last_expr = None;
      let valid = parser
        .get_statement(
          &mut LoopArgument::with_tokens(parser.arena, "ternary_valid", &[], &[TokenType::Colon])
        )?
        .ok_internal()?;
      parser.position += 1;
      let invalid = parser
        .get_statement(
          &mut LoopArgument::safe(
            parser.arena,
            "ternary_invalid",
            &[],
            &args.breakers.combine(args.separators).combine(&[TokenType::Semicolon]),
            &DEFAULT_PARSERS
          )
        )?
        .ok_internal()?;
      return Ok(
        TernaryNode::loc(
          left.into_boxed(parser.arena),
          valid.into_boxed(parser.arena),
          invalid.into_boxed(parser.arena),
          parser.gen_loc(start_loc)
        )
      );
    }
    Err(ParserError::Internal)
  }
}
