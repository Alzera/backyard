mod internal;
mod parser;
mod utils;
pub mod error;

use backyard_lexer::{ lex_byte_in, token::Token };
use backyard_nodes::{ serde::node::SerializableNode, Location, Node, ProgramNode, RangeLocation };
use bumpalo::Bump;
use error::ParserError;
use parser::{ LocationHelper, LoopArgument, Parser };

pub fn parse(is_eval: bool, input: &str) -> Result<SerializableNode, ParserError> {
  let arena = Bump::new();
  let result = parse_byte_in(&arena, is_eval, input.as_bytes())?;
  Ok(result.serializable())
}

pub fn parse_in<'arena>(
  arena: &'arena Bump,
  is_eval: bool,
  input: &str
) -> Result<Node<'arena>, ParserError> {
  parse_byte_in(arena, is_eval, input.as_bytes())
}

pub fn parse_byte(is_eval: bool, input: &[u8]) -> Result<SerializableNode, ParserError> {
  let arena = Bump::new();
  let result = parse_byte_in(&arena, is_eval, input)?;
  Ok(result.serializable())
}

pub fn parse_byte_in<'arena>(
  arena: &'arena Bump,
  is_eval: bool,
  input: &[u8]
) -> Result<Node<'arena>, ParserError> {
  let tokens = lex_byte_in(arena, is_eval, input).map_err(|x| ParserError::LexError(x.to_owned()))?;
  parse_tokens_in(arena, &tokens)
}

pub fn parse_tokens<'arena>(
  tokens: &bumpalo::collections::Vec<'arena, Token>
) -> Result<SerializableNode, ParserError> {
  let arena = Bump::new();
  let result = parse_tokens_in(&arena, tokens)?;
  Ok(result.serializable())
}

pub fn parse_tokens_in<'arena>(
  arena: &'arena Bump,
  tokens: &bumpalo::collections::Vec<'arena, Token>
) -> Result<Node<'arena>, ParserError> {
  if tokens.is_empty() {
    return Err(ParserError::Eof);
  }
  let mut parser = Parser::new(arena, tokens);
  Ok(
    ProgramNode::loc(
      parser.get_children(&mut LoopArgument::default(arena, "main"))?,
      Some(RangeLocation {
        start: Location { line: 1, column: 0, offset: 0 },
        end: tokens.last().unwrap().get_location().unwrap(),
      })
    )
  )
}
