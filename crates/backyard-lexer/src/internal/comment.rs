use crate::error::LexResult;
use crate::lexer::Lexer;
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

  pub fn lex_doc(lexer: &mut Lexer, t: &str) -> LexResult {
    let comment = CommentToken::parse(lexer);
    let mut t = t.to_string();
    t.push_str(&comment);
    Ok(vec![Token::new(TokenType::CommentDoc, t)])
  }

  pub fn lex_block(lexer: &mut Lexer, t: &str) -> LexResult {
    let comment = CommentToken::parse(lexer);
    let mut t = t.to_string();
    t.push_str(&comment);
    Ok(vec![Token::new(TokenType::CommentBlock, t)])
  }

  pub fn lex_line(lexer: &mut Lexer, t: &str) -> LexResult {
    let comment = lexer.control.next_char_until(|_, ch, _| ['\n'].contains(ch));
    println!("comment {}", comment);
    let mut t = t.to_string();
    t.push_str(&comment);
    println!("t {}", t);
    Ok(vec![Token::new(TokenType::CommentLine, t)])
  }
}
