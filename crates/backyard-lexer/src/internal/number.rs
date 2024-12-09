use compact_str::format_compact;

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
        let n = format_compact!("0x{}", t);
        return {
            lexer.tokens.push(Token::new(TokenType::NumberHex, n, snapshot));
            Ok(())
        };
      }
      if next == 'b' {
        lexer.control.next_char();
        let t = lexer.control.next_char_until(|_, ch, _| !(*ch == '0' || *ch == '1' || *ch == '_'));
        let n = format_compact!("0b{}", t);
        return {
            lexer.tokens.push(Token::new(TokenType::NumberBinary, n, snapshot));
            Ok(())
        };
      }
    }
    let mut t = lexer.control.next_char_until(|_, ch, _| !(ch.is_ascii_digit() || *ch == '.'));
    t.insert(0, current_char);
    lexer.tokens.push(Token::new(TokenType::Number, t, snapshot));
    Ok(())
  }
}
