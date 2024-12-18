pub mod token;
mod internal;
mod lexer;
pub mod error;

use bstr::BString;
use bumpalo::Bump;
use error::LexError;
use lexer::Lexer;
use token::Token;

pub fn byte_lex<'a>(is_eval: bool, input: &[u8]) -> Result<Vec<Token>, LexError> {
  let arena = Bump::new();
  let result = arena_byte_lex(&arena, is_eval, input)?;
  Ok(Vec::from_iter(result.into_iter()))
}

pub fn arena_byte_lex<'a>(
  arena: &'a Bump,
  is_eval: bool,
  input: &[u8]
) -> Result<bumpalo::collections::Vec<'a, Token>, LexError> {
  let mut lexer = Lexer::new(arena, BString::new(input.to_vec()));
  lexer.start(is_eval)?;
  Ok(lexer.tokens)
}

pub fn lex<'a>(is_eval: bool, input: &str) -> Result<Vec<Token>, LexError> {
  let arena = Bump::new();
  let result = arena_lex(&arena, is_eval, input)?;
  Ok(Vec::from_iter(result.into_iter()))
}

pub fn arena_lex<'a>(
  arena: &'a Bump,
  is_eval: bool,
  input: &str
) -> Result<bumpalo::collections::Vec<'a, Token>, LexError> {
  let mut lexer = Lexer::new(arena, BString::new(input.as_bytes().to_vec()));
  lexer.start(is_eval)?;
  Ok(lexer.tokens)
}
