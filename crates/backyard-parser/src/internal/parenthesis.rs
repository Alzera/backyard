use backyard_lexer::token::TokenType;
use backyard_nodes::{ CastNode, Location, Node, ParenthesisNode, utils::IntoBoxedNode };

use crate::{
  error::ParserError,
  guard,
  parser::{ LoopArgument, Parser, DEFAULT_PARSERS },
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
      if let Some(token) = parser.tokens.get(parser.position) {
        if CAST_TYPES.contains(&token.value.as_slice()) {
          if let Some(next_token) = parser.tokens.get(parser.position + 1) {
            if next_token.token_type == TokenType::RightParenthesis {
              parser.position += 2;
              let expression = guard!(
                parser.get_statement(
                  &mut LoopArgument::safe(
                    parser.arena,
                    "cast",
                    args.separators,
                    args.breakers,
                    &DEFAULT_PARSERS
                  )
                )?
              );
              return Ok(
                CastNode::loc(
                  token.value.to_owned(),
                  expression.into_boxed(parser.arena),
                  parser.gen_loc(start_loc)
                )
              );
            }
          }
        }
      }
      let statement = guard!(
        parser.get_statement(
          &mut LoopArgument::with_tokens(
            parser.arena,
            "parenthesis",
            &[],
            &[TokenType::RightParenthesis]
          )
        )?
      );
      parser.position += 1;
      return Ok(
        ParenthesisNode::loc(statement.into_boxed(parser.arena), parser.gen_loc(start_loc))
      );
    }
    Err(ParserError::Internal)
  }
}
