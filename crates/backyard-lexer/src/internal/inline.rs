use crate::error::LexResult;
use crate::lexer::{ Lexer, SeriesChecker };
use crate::token::{ Token, TokenType };

pub struct InlineToken;

impl InlineToken {
  pub fn lex(lexer: &mut Lexer, token: Option<Token>) -> LexResult {
    let mut result = vec![];
    if token.is_some() {
      result.push(token.unwrap());
    }
    let mut checker = SeriesChecker::safe(&["<?php", "<?=", "<%"]);
    let max_index = lexer.control.get_len().saturating_sub(1);
    let mut no_breaker = false;
    let mut inline = lexer.control.next_char_until(|_, ch, i| {
      if *i >= max_index {
        *i += 1;
        no_breaker = true;
        return true;
      }
      checker.push(*ch);
      checker.check().is_some()
    });
    lexer.control.next_char();
    if no_breaker {
      if inline.len() > 0 {
        result.push(Token::new(TokenType::Inline, inline));
      }
    } else if let Some(breaker) = checker.check() {
      inline = inline[..inline.len() - breaker[..breaker.len() - 1].len()].to_string();
      if inline.len() > 0 {
        result.push(Token::new(TokenType::Inline, inline));
      }
      match breaker {
        "<%" => result.push(Token::new(TokenType::OpenTagShort, "<%")),
        "<?php" => result.push(Token::new(TokenType::OpenTag, "<?php")),
        "<?=" => result.push(Token::new(TokenType::OpenTagEcho, "<?=")),
        _ => (),
      }
    }
    Ok(result)
  }
}
