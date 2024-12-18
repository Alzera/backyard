pub mod token;
mod internal;
mod lexer;
pub mod error;

use bstr::BString;
use bumpalo::Bump;
use error::LexError;
use lexer::Lexer;
use token::Token;

pub fn lex(is_eval: bool, input: &str) -> Result<Vec<Token>, LexError> {
  let arena = Bump::new();
  let result = lex_in(&arena, is_eval, input)?;
  Ok(Vec::from_iter(result))
}

pub fn lex_in<'arena>(
  arena: &'arena Bump,
  is_eval: bool,
  input: &str
) -> Result<bumpalo::collections::Vec<'arena, Token>, LexError> {
  let mut lexer = Lexer::new(arena, BString::new(input.as_bytes().to_vec()));
  lexer.start(is_eval)?;
  Ok(lexer.tokens)
}

pub fn lex_byte(is_eval: bool, input: &[u8]) -> Result<Vec<Token>, LexError> {
  let arena = Bump::new();
  let result = lex_byte_in(&arena, is_eval, input)?;
  Ok(Vec::from_iter(result))
}

pub fn lex_byte_in<'arena>(
  arena: &'arena Bump,
  is_eval: bool,
  input: &[u8]
) -> Result<bumpalo::collections::Vec<'arena, Token>, LexError> {
  let mut lexer = Lexer::new(arena, BString::new(input.to_vec()));
  lexer.start(is_eval)?;
  Ok(lexer.tokens)
}
