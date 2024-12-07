pub mod token;
mod internal;
mod lexer;
pub mod error;

use error::LexResult;
use internal::inline::InlineToken;
use lexer::{ ControlSnapshot, Lexer };
use token::Token;

pub fn lex(input: &str) -> LexResult {
  let mut lexer = Lexer::new(input);
  let mut tokens: Vec<Token> = Vec::new();
  let inline = InlineToken::lex(
    &mut lexer,
    &(ControlSnapshot { line: 1, column: 0, offset: 0 })
  ).unwrap();
  tokens.extend(inline);
  tokens.extend(lexer.start()?);
  Ok(tokens)
}

pub fn lex_eval(input: &str) -> LexResult {
  Lexer::new(input).start()
}
