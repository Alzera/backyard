mod internal;
mod parser;
mod utils;
pub mod error;

use backyard_lexer::{ arena_lex, error::LexError, token::Token };
use backyard_nodes::{ serde::SerializableNode, Location, Node, ProgramNode, RangeLocation };
use bumpalo::Bump;
use error::ParserError;
use parser::{ LocationHelper, LoopArgument, Parser };

pub fn parse<'arena>(is_eval: bool, input: &str) -> Result<SerializableNode, ParserError> {
  let arena = Bump::new();
  let result = arena_parse(&arena, is_eval, input)?;
  Ok(result.serializable())
}

pub fn arena_parse<'arena>(
  arena: &'arena Bump,
  is_eval: bool,
  input: &str
) -> Result<Node<'arena>, ParserError> {
  let tokens = arena_lex(arena, is_eval, input);
  parse_base(arena, &tokens)
}

pub fn parse_base<'arena>(
  arena: &'arena Bump,
  tokens: &Result<bumpalo::collections::Vec<'arena, Token>, LexError>
) -> Result<Node<'arena>, ParserError> {
  match tokens {
    Ok(tokens) => {
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
    Err(err) => Err(ParserError::LexError(err.to_owned())),
  }
}
