pub mod token;
mod internal;
mod lexer;
pub mod error;

use bumpalo::Bump;
use error::LexError;
use internal::inline::InlineToken;
use lexer::{ ControlSnapshot, Lexer };
use token::Token;

pub fn lex(input: &str) -> Result<Vec<Token>, LexError> {
  let arena = Bump::new();
  let result = arena_lex(&arena, input)?;
  Ok(result.to_vec())
}

pub fn lex_eval(input: &str) -> Result<Vec<Token>, LexError> {
  let arena = Bump::new();
  let result = arena_lex_eval(&arena, input)?;
  Ok(result.to_vec())
}

pub fn arena_lex<'a>(
  arena: &'a Bump,
  input: &str
) -> Result<bumpalo::collections::Vec<'a, Token>, LexError> {
  let mut lexer = Lexer::new(arena, input);
  InlineToken::lex(&mut lexer, &(ControlSnapshot { line: 1, column: 0, offset: 0 }))?;
  lexer.start()?;
  Ok(lexer.tokens)
}

pub fn arena_lex_eval<'a>(
  arena: &'a Bump,
  input: &str
) -> Result<bumpalo::collections::Vec<'a, Token>, LexError> {
  let mut lexer = Lexer::new(arena, input);
  lexer.start()?;
  Ok(lexer.tokens)
}
