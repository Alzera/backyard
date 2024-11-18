pub mod token;
mod internal;
pub mod lexer;
mod utils;

use lexer::Lexer;
use token::Token;

pub fn lex(input: &str) -> Vec<Token> {
  let mut lexer = Lexer::new(&input);
  let mut tokens: Vec<Token> = Vec::new();
  while let Some(token) = lexer.next_tokens(true) {
    tokens.extend(token);
  }
  tokens
}
