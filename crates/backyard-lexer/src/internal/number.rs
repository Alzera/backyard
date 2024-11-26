use crate::error::LexResult;
use crate::lexer::{ ControlSnapshot, Lexer };
use crate::token::{ Token, TokenType };

pub struct NumberToken;

impl NumberToken {
  pub fn lex(lexer: &mut Lexer, current_char: char, snapshot: &ControlSnapshot) -> LexResult {
    if let Some(next) = lexer.control.peek_char(None) {
      if next == 'x' {
        lexer.control.next_char();
        let t = lexer.control.next_char_until(|_, ch, _| !ch.is_alphanumeric());
        let mut n = "0x".to_string();
        n.push_str(&t);
        return Ok(vec![Token::new(TokenType::NumberHex, n, snapshot)]);
      }
    }
    let mut t = lexer.control.next_char_until(|_, ch, _| !(ch.is_ascii_digit() || *ch == '.'));
    t.insert(0, current_char);
    Ok(vec![Token::new(TokenType::Number, t, snapshot)])
  }
}
