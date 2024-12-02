use crate::error::LexResult;
use crate::lexer::{ ControlSnapshot, Lexer, SeriesChecker, SeriesCheckerMode };
use crate::token::{ Token, TokenType };

pub struct InlineToken;

impl InlineToken {
  pub fn lex(lexer: &mut Lexer, snapshot: &ControlSnapshot) -> LexResult {
    let mut result = vec![];
    let mut checker = SeriesChecker::new(&["<?php", "<?=", "<%"], SeriesCheckerMode::Inline);
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
      if !inline.is_empty() {
        result.push(Token::new(TokenType::Inline, inline, snapshot));
      }
    } else if let Some(breaker) = checker.check() {
      inline = inline[..inline.len() - breaker[..breaker.len() - 1].len()].to_string();
      if !inline.is_empty() {
        result.push(Token::new(TokenType::Inline, inline, snapshot));
      }
      if breaker == "<?=" {
        result.push(Token::new(TokenType::Echo, "echo", snapshot));
      }
    }
    Ok(result)
  }
}
