use backyard_lexer::token::TokenType;
use backyard_nodes::{ IncludeNode, Location, Node, utils::IntoBoxedNode };

use crate::{
  error::ParserError,
  parser::{ LoopArgument, OptionNodeOrInternal, Parser, TokenTypeArrayCombine },
  utils::{ match_pattern, Lookup, LookupResult },
};

#[derive(Debug, Clone)]
pub struct IncludeParser;

impl IncludeParser {
  pub fn test<'arena, 'a>(
    parser: &mut Parser<'arena, 'a>,
    _: &mut LoopArgument
  ) -> Option<std::vec::Vec<LookupResult<'arena>>> {
    match_pattern(
      parser,
      &[
        Lookup::Equal(
          &[TokenType::Require, TokenType::RequireOnce, TokenType::Include, TokenType::IncludeOnce]
        ),
        Lookup::Optional(&[TokenType::LeftParenthesis]),
      ]
    )
  }

  pub fn parse<'arena, 'a, 'b>(
    parser: &mut Parser<'arena, 'a>,
    matched: std::vec::Vec<LookupResult<'arena>>,
    start_loc: Location,
    args: &mut LoopArgument<'arena, 'b>
  ) -> Result<Node<'arena>, ParserError> {
    if let [keyword, use_parenthesis] = matched.as_slice() {
      let keyword = keyword.as_equal(parser)?;
      let is_require =
        keyword.token_type == TokenType::Require || keyword.token_type == TokenType::RequireOnce;
      let is_once =
        keyword.token_type == TokenType::RequireOnce ||
        keyword.token_type == TokenType::IncludeOnce;
      let use_parenthesis = !use_parenthesis.is_empty();
      let argument = (
        if use_parenthesis {
          let a = parser.get_statement(
            &mut LoopArgument::with_tokens(
              parser.arena,
              "include",
              &[],
              &[TokenType::RightParenthesis]
            )
          )?;
          parser.position += 1;
          a
        } else {
          parser.get_statement(
            &mut LoopArgument::with_tokens(
              parser.arena,
              "include",
              &[],
              &args.breakers.combine(args.separators)
            )
          )?
        }
      ).ok_internal()?;
      return Ok(
        IncludeNode::loc(
          use_parenthesis,
          is_require,
          is_once,
          argument.into_boxed(parser.arena),
          parser.gen_loc(start_loc)
        )
      );
    }
    Err(ParserError::Internal)
  }
}
