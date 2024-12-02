use crate::error::LexResult;
use crate::lexer::{ ControlSnapshot, Lexer };
use crate::token::{ Token, TokenType };

pub struct CommentToken;

impl CommentToken {
  pub fn parse(lexer: &mut Lexer) -> String {
    let mut close: Vec<char> = Vec::new();
    let comment = lexer.control.next_char_until(|_, ch, endpos| {
      close.push(*ch);
      if close.len() > 2 {
        close.remove(0);
      }
      let is_close = close.iter().collect::<String>() == "*/";
      if is_close {
        *endpos -= 1;
      }
      is_close
    });
    lexer.control.next_char();
    lexer.control.next_char();
    comment
  }

  pub fn lex_doc(lexer: &mut Lexer, t: &str, snapshot: &ControlSnapshot) -> LexResult {
    let comment = CommentToken::parse(lexer);
    let mut t = t.to_string();
    t.push_str(&comment);
    Ok(vec![Token::new(TokenType::CommentDoc, t, snapshot)])
  }

  pub fn lex_block(lexer: &mut Lexer, t: &str, snapshot: &ControlSnapshot) -> LexResult {
    let comment = CommentToken::parse(lexer);
    let mut t = t.to_string();
    t.push_str(&comment);
    Ok(vec![Token::new(TokenType::CommentBlock, t, snapshot)])
  }

  pub fn lex_line(lexer: &mut Lexer, t: &str, snapshot: &ControlSnapshot) -> LexResult {
    let comment = {
      lexer.control.next_char_until(|control, ch, i| {
        if *ch == '\n' {
          return true;
        }
        if let Some(next_char) = control.peek_char(Some(*i + 1)) {
          if *ch == '?' && next_char == '>' {
            return true;
          }
        }
        return false;
      })
    };
    let mut t = t.to_string();
    t.push_str(&comment);
    Ok(vec![Token::new(TokenType::CommentLine, t, snapshot)])
  }
}
