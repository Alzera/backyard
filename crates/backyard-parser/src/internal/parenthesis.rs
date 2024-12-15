use backyard_lexer::token::TokenType;
use backyard_nodes::{ utils::IntoBoxedNode, CastNode, CastType, Location, Node, ParenthesisNode };

use crate::{
  error::ParserError,
  parser::{ LoopArgument, OptionNodeOrInternal, Parser, DEFAULT_PARSERS },
  utils::{ match_pattern, Lookup, LookupResult },
};

#[derive(Debug, Clone)]
pub struct ParenthesisParser;

const CAST_TYPES: &[&[u8]] = &[
  b"int",
  b"integer",
  b"bool",
  b"boolean",
  b"float",
  b"double",
  b"real",
  b"string",
  b"binary",
  b"array",
  b"object",
  b"unset",
];

impl ParenthesisParser {
  pub fn test<'arena, 'a>(
    parser: &mut Parser<'arena, 'a>,
    _: &mut LoopArgument
  ) -> Option<std::vec::Vec<LookupResult<'arena>>> {
    match_pattern(parser, &[Lookup::Equal(&[TokenType::LeftParenthesis])])
  }

  pub fn parse<'arena, 'a, 'b>(
    parser: &mut Parser<'arena, 'a>,
    matched: std::vec::Vec<LookupResult<'arena>>,
    start_loc: Location,
    args: &mut LoopArgument<'arena, 'b>
  ) -> Result<Node<'arena>, ParserError> {
    if let [_] = matched.as_slice() {
      if let Ok(token) = parser.get_token(parser.position) {
        if CAST_TYPES.contains(&token.value.as_slice()) {
          if let Ok(next_token) = parser.get_token(parser.position + 1) {
            if next_token.token_type == TokenType::RightParenthesis {
              let cast_type = CastType::try_from(&token.value).map_err(|_| ParserError::Internal)?;
              parser.position += 2;
              let expression = parser
                .get_statement(
                  &mut LoopArgument::safe(
                    parser.arena,
                    "cast",
                    args.separators,
                    args.breakers,
                    &DEFAULT_PARSERS
                  )
                )?
                .ok_internal()?;
              return Ok(
                CastNode::loc(
                  cast_type,
                  expression.into_boxed(parser.arena),
                  parser.gen_loc(start_loc)
                )
              );
            }
          }
        }
      }
      let statement = parser
        .get_statement(
          &mut LoopArgument::with_tokens(
            parser.arena,
            "parenthesis",
            &[],
            &[TokenType::RightParenthesis]
          )
        )?
        .ok_internal()?;
      parser.position += 1;
      return Ok(
        ParenthesisNode::loc(statement.into_boxed(parser.arena), parser.gen_loc(start_loc))
      );
    }
    Err(ParserError::Internal)
  }
}
