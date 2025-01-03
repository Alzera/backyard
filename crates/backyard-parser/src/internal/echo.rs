use backyard_lexer::token::TokenType;
use backyard_nodes::{ EchoNode, Location, Node };

use crate::{
  error::ParserError,
  parser::{ LoopArgument, Parser, TokenTypeArrayCombine },
  utils::{ match_pattern, Lookup, LookupResult },
};

#[derive(Debug, Clone)]
pub struct EchoParser;

impl EchoParser {
  pub fn test<'arena, 'a>(
    parser: &mut Parser<'arena, 'a>,
    _: &mut LoopArgument
  ) -> Option<std::vec::Vec<LookupResult<'arena>>> {
    match_pattern(parser, &[Lookup::Equal(&[TokenType::Echo])])
  }

  pub fn parse<'arena, 'a, 'b>(
    parser: &mut Parser<'arena, 'a>,
    matched: std::vec::Vec<LookupResult<'arena>>,
    start_loc: Location,
    args: &mut LoopArgument<'arena, 'b>
  ) -> Result<Node<'arena>, ParserError> {
    if let [_] = matched.as_slice() {
      let items = parser.get_children(
        &mut LoopArgument::with_tokens(
          parser.arena,
          "echo",
          &[TokenType::Comma],
          &args.breakers
            .combine(args.separators)
            .combine(&[TokenType::Semicolon, TokenType::Inline])
        )
      )?;
      if let Ok(last_token) = parser.get_token(parser.position - 1) {
        if [TokenType::Semicolon, TokenType::Inline].contains(&last_token.token_type) {
          parser.position -= 1;
        }
      }
      return Ok(EchoNode::loc(items, parser.gen_loc(start_loc)));
    }
    Err(ParserError::Internal)
  }
}
