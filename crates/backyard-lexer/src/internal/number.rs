use crate::error::LexResult;
use crate::lexer::{ ControlSnapshot, Lexer };
use crate::token::{ Token, TokenType };

pub struct NumberToken;

impl NumberToken {
  pub fn lex(lexer: &mut Lexer, current_char: &u8, snapshot: &ControlSnapshot) -> LexResult {
    if *current_char == b'0' {
      if let Some(next) = lexer.control.peek_char(None) {
        if *next == b'x' {
          lexer.control.next_char();
          let t = lexer.control.next_char_until(2, |_, ch, _| !ch.is_ascii_alphanumeric());
          return {
            lexer.tokens.push(Token::new(TokenType::NumberHex, t, snapshot));
            Ok(())
          };
        }
        if *next == b'b' {
          lexer.control.next_char();
          let t = lexer.control.next_char_until(
            2,
            |_, ch, _| !(ch == b'0' || ch == b'1' || ch == b'_')
          );
          return {
            lexer.tokens.push(Token::new(TokenType::NumberBinary, t, snapshot));
            Ok(())
          };
        }
      }
    }
    let t = lexer.control.next_char_until(1, |_, ch, _| !(ch.is_ascii_digit() || ch == b'.'));
    lexer.tokens.push(Token::new(TokenType::Number, t, snapshot));
    Ok(())
  }
}
