use compact_str::CompactString;

use crate::error::LexResult;
use crate::lexer::{ ControlSnapshot, Lexer, SeriesChecker, SeriesCheckerMode };
use crate::token::{ Token, TokenType };

pub struct InlineToken;

impl InlineToken {
  pub fn lex(lexer: &mut Lexer, snapshot: &ControlSnapshot) -> LexResult {
    let mut checker = SeriesChecker::new(&["<?php", "<?=", "<%"], SeriesCheckerMode::Inline);
    let max_index = lexer.control.get_len().saturating_sub(1);
    let mut no_breaker = false;
    let inline = lexer.control.next_char_until(0, |_, ch, i| {
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
        lexer.tokens.push(Token::new(TokenType::Inline, inline, snapshot));
      }
    } else if let Some(breaker) = checker.check() {
      let inline: CompactString =
        inline[..inline.len() - breaker[..breaker.len() - 1].len()].into();
      if !inline.is_empty() {
        lexer.tokens.push(Token::new(TokenType::Inline, inline, snapshot));
      }
      if breaker == "<?=" {
        lexer.tokens.push(Token::new(TokenType::Echo, "echo".into(), snapshot));
      }
    }
    Ok(())
  }
}
