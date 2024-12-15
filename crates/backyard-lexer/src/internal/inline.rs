use bstr::BString;

use crate::error::LexResult;
use crate::lexer::{ ControlSnapshot, Lexer, SeriesChecker, SeriesCheckerMode };
use crate::token::{ Token, TokenType };

pub struct InlineToken;

impl InlineToken {
  pub fn lex(lexer: &mut Lexer, snapshot: &ControlSnapshot) -> LexResult {
    let againsts = [b"<?php".into(), b"<?=".into(), b"<%".into()];
    let mut checker = SeriesChecker::new(&againsts, SeriesCheckerMode::Inline);
    let inline = lexer.control.next_char_until(0, |_, ch, _| {
      checker.push(ch);
      checker.check().is_some()
    });
    if let Some(breaker) = checker.check() {
      lexer.control.next_char();
      let inline: BString = inline[..inline.len() - breaker[..breaker.len() - 1].len()].into();
      if !inline.is_empty() {
        lexer.tokens.push(Token::new(TokenType::Inline, inline, snapshot));
      }
      if breaker == "<?=" {
        lexer.tokens.push(Token::new(TokenType::Echo, "echo".into(), snapshot));
      }
    } else if !inline.is_empty() {
      lexer.tokens.push(Token::new(TokenType::Inline, inline, snapshot));
    }
    Ok(())
  }
}
