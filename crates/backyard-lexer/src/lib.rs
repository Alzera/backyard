pub mod token;
mod internal;
mod lexer;
pub mod error;

use bstr::BString;
use bumpalo::Bump;
use error::LexError;
use internal::inline::InlineToken;
use lexer::{ ControlSnapshot, Lexer };
use token::Token;

pub fn bytes_lex<'a>(
  arena: &'a Bump,
  input: Vec<u8>
) -> Result<bumpalo::collections::Vec<'a, Token>, LexError> {
  let mut lexer = Lexer::new(arena, BString::new(input));
  InlineToken::lex(&mut lexer, &(ControlSnapshot { line: 1, column: 0, offset: 0 }))?;
  lexer.start()?;
  Ok(lexer.tokens)
}

pub fn bytes_lex_eval<'a>(
  arena: &'a Bump,
  input: Vec<u8>
) -> Result<bumpalo::collections::Vec<'a, Token>, LexError> {
  let mut lexer = Lexer::new(arena, BString::new(input));
  lexer.start()?;
  Ok(lexer.tokens)
}

pub fn lex<'a>(
  arena: &'a Bump,
  input: &str
) -> Result<bumpalo::collections::Vec<'a, Token>, LexError> {
  bytes_lex(arena, input.as_bytes().to_vec())
}

pub fn lex_eval<'a>(
  arena: &'a Bump,
  input: &str
) -> Result<bumpalo::collections::Vec<'a, Token>, LexError> {
  bytes_lex_eval(arena, input.as_bytes().to_vec())
}
