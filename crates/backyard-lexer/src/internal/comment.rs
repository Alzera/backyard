use bstr::BString;

use crate::error::LexResult;
use crate::lexer::{ ControlSnapshot, Lexer };
use crate::token::{ Token, TokenType };

pub struct CommentToken;

impl CommentToken {
  pub fn parse(lexer: &mut Lexer, take_prev_len: usize) -> BString {
    let mut close: Vec<u8> = Vec::new();
    let comment = lexer.control.next_char_until(take_prev_len, |_, ch, endpos| {
      close.push(ch);
      if close.len() > 2 {
        close.remove(0);
      }
      let is_close = close.get(0) == Some(&b'*') && close.get(1) == Some(&b'/');
      if is_close {
        *endpos -= 1;
      }
      is_close
    });
    lexer.control.consume(2);
    comment
  }

  pub fn lex_doc(lexer: &mut Lexer, take_prev_len: usize, snapshot: &ControlSnapshot) -> LexResult {
    let comment = CommentToken::parse(lexer, take_prev_len);
    lexer.tokens.push(Token::new(TokenType::CommentDoc, comment, snapshot));
    Ok(())
  }

  pub fn lex_block(
    lexer: &mut Lexer,
    take_prev_len: usize,
    snapshot: &ControlSnapshot
  ) -> LexResult {
    let comment = CommentToken::parse(lexer, take_prev_len);
    lexer.tokens.push(Token::new(TokenType::CommentBlock, comment, snapshot));
    Ok(())
  }

  pub fn lex_line(
    lexer: &mut Lexer,
    take_prev_len: usize,
    snapshot: &ControlSnapshot
  ) -> LexResult {
    let comment = {
      lexer.control.next_char_until(take_prev_len, |control, ch, i| {
        if ch == b'\n' {
          return true;
        }
        if let Some(next_char) = control.peek_char(Some(*i + 1)) {
          if ch == b'?' && *next_char == b'>' {
            return true;
          }
        }
        false
      })
    };
    lexer.tokens.push(Token::new(TokenType::CommentLine, comment, snapshot));
    Ok(())
  }
}
