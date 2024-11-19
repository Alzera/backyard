pub mod token;
mod internal;
pub mod lexer;
pub mod error;

use error::{ LexError, LexResult };
use lexer::Lexer;
use token::Token;

pub fn lex(input: &str) -> LexResult {
  let mut lexer = Lexer::new(&input);
  let mut tokens: Vec<Token> = Vec::new();
  loop {
    match lexer.next_tokens(true) {
      Ok(token) => tokens.extend(token),
      Err(err) => {
        if err == LexError::Eof {
          break;
        }
        return Err(err);
      }
    }
  }
  Ok(tokens)
}
