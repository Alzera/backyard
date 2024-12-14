pub mod token;
mod internal;
mod lexer;
pub mod error;

use bstr::BString;
use bumpalo::Bump;
use error::LexError;
use lexer::Lexer;
use token::Token;

pub fn lex_bytes<'a>(
  arena: &'a Bump,
  input: Vec<u8>
) -> Result<bumpalo::collections::Vec<'a, Token>, LexError> {
  let mut lexer = Lexer::new(arena, BString::new(input));
  lexer.start(true)?;
  Ok(lexer.tokens)
}

pub fn lex_eval_bytes<'a>(
  arena: &'a Bump,
  input: Vec<u8>
) -> Result<bumpalo::collections::Vec<'a, Token>, LexError> {
  let mut lexer = Lexer::new(arena, BString::new(input));
  lexer.start(false)?;
  Ok(lexer.tokens)
}

pub fn lex<'a>(
  arena: &'a Bump,
  input: &str
) -> Result<bumpalo::collections::Vec<'a, Token>, LexError> {
  lex_bytes(arena, input.as_bytes().to_vec())
}

pub fn lex_eval<'a>(
  arena: &'a Bump,
  input: &str
) -> Result<bumpalo::collections::Vec<'a, Token>, LexError> {
  lex_eval_bytes(arena, input.as_bytes().to_vec())
}
