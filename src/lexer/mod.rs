pub mod token;
mod internal;
mod lexer;
mod utils;

use crate::lexer::lexer::Lexer;
use crate::lexer::token::Token;

pub fn lex(input: String) -> Vec<Token> {
  let mut lexer = Lexer::new(&input);
  let mut tokens: Vec<Token> = Vec::new();
  while let Some(token) = lexer.next_tokens(true) {
    tokens.extend(token);
  }
  tokens
}
