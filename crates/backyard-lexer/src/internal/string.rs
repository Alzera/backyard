use compact_str::{ format_compact, CompactString, ToCompactString };

use crate::error::{ LexError, LexResult };
use crate::internal::variable::VariableToken;
use crate::lexer::{ ControlSnapshot, Lexer, SeriesChecker, SeriesCheckerMode };
use crate::token::{ Token, TokenType };

pub struct StringToken;

impl StringToken {
  fn get_parts(
    lexer: &mut Lexer,
    breaker: &str,
    mode: SeriesCheckerMode
  ) -> Result<bool, LexError> {
    let mut token_count = 0;
    let againsts = [breaker];
    let mut checker = SeriesChecker::new(&againsts, mode);
    let mut need_check_condition: Vec<char> = breaker.chars().collect();
    need_check_condition.push('$');
    need_check_condition.push('{');
    loop {
      let snapshot = lexer.control.get_snapshot();
      let mut t = lexer.control.next_char_until(|control, ch, end_position| {
        checker.push(*ch);
        if need_check_condition.contains(ch) {
          if checker.check().is_some() {
            return true;
          } else if *ch == '$' {
            if checker.is_last_escaped() {
              return false;
            }
            if let Some(next) = control.peek_char(Some(*end_position + 1)) {
              if next == '_' || next.is_alphabetic() {
                return true;
              }
            }
          } else if *ch == '{' {
            if checker.is_last_escaped() {
              return false;
            }
            if let Some(next) = control.peek_char(Some(*end_position + 1)) {
              if next == '$' {
                if let Some(next) = control.peek_char(Some(*end_position + 2)) {
                  if next == '_' || next == '{' || next.is_alphabetic() {
                    return true;
                  }
                }
              }
            }
          }
        }
        false
      });
      token_count += 1;
      let current = if let Some(current) = lexer.control.peek_char(None) {
        current
      } else {
        break;
      };
      if checker.check().is_some() {
        t.push(current);
        t = t[..t.len() - breaker.len()].into();
        lexer.tokens.push(Token::new(TokenType::EncapsedString, t, &snapshot));
        lexer.control.next_char();
        break;
      }
      if !t.is_empty() {
        lexer.tokens.push(Token::new(TokenType::EncapsedString, t, &snapshot));
      }

      let next = lexer.control.peek_char(Some(lexer.control.get_position() + 1));
      let snapshot = lexer.control.get_snapshot();

      if current == '$' {
        lexer.control.next_char();
        VariableToken::lex(lexer, &snapshot)?;
      } else if next.is_some() && current == '{' && next.unwrap() == '$' {
        lexer.control.next_char();
        lexer.tokens.push(Token::new(TokenType::AdvanceInterpolationOpen, "{".into(), &snapshot));
        lexer.next_tokens_until_right_bracket()?;
        lexer.control.next_char();
        lexer.tokens.push(
          Token::new(
            TokenType::AdvanceInterpolationClose,
            "}".into(),
            lexer.control.get_last_snapshot()
          )
        );
      }
    }
    Ok(token_count == 1)
  }

  pub fn lex_basic(lexer: &mut Lexer, breaker: &str, snapshot: &ControlSnapshot) -> LexResult {
    let checker_breaker: [&str; 1] = [breaker];
    let mut checker = SeriesChecker::new(&checker_breaker, SeriesCheckerMode::String);
    let text = lexer.control.next_char_until(|_, i, _| {
      checker.push(*i);
      checker.check().is_some()
    });
    lexer.control.next_char();

    lexer.tokens.push(
      Token::new(TokenType::String, format_compact!("{}{}{}", breaker, text, breaker), snapshot)
    );
    Ok(())
  }

  pub fn lex(lexer: &mut Lexer, breaker: &str, snapshot: &ControlSnapshot) -> LexResult {
    let breaker = breaker.to_compact_string();
    lexer.tokens.push(Token::new(TokenType::EncapsedStringOpen, breaker.clone(), snapshot));

    let is_without_encapsed = Self::get_parts(lexer, &breaker, SeriesCheckerMode::String)?;

    if is_without_encapsed {
      if let Some(string_token) = lexer.tokens.pop() {
        lexer.tokens.pop();
        lexer.tokens.push(
          Token::new(
            TokenType::String,
            format_compact!("{}{}{}", breaker, string_token.value, breaker),
            snapshot
          )
        );
        return Ok(());
      }
    }

    lexer.tokens.push(
      Token::new(TokenType::EncapsedStringClose, breaker, lexer.control.get_last_snapshot())
    );

    Ok(())
  }

  pub fn lex_doc(lexer: &mut Lexer, snapshot: &ControlSnapshot) -> LexResult {
    let label = lexer.control
      .next_char_until(|_, i, _| *i == '\n')
      .trim()
      .to_compact_string();
    if
      !label
        .chars()
        .enumerate()
        .fold(
          true,
          |acc, (i, ch)|
            acc &&
            (ch.is_alphanumeric() ||
              ch == '_' ||
              ((i == 0 || i == label.len() - 1) && ['\'', '"'].contains(&ch)))
        )
    {
      return Err(lexer.control.error_unrecognized(&label));
    }
    if label.starts_with('\'') && label.ends_with('\'') {
      let clean_label: CompactString = label
        .get(1..label.len() - 1)
        .unwrap_or_default()
        .into();
      let againsts: [&str; 1] = [clean_label.as_str()];
      let mut checker = SeriesChecker::new(&againsts, SeriesCheckerMode::Heredoc);
      let mut should_break = false;
      let content_snapshot = lexer.control.get_snapshot();
      let text = lexer.control.next_char_until(|_, i, _| {
        checker.push(*i);
        let t = should_break;
        should_break = checker.check().is_some();
        t
      });
      let text = text[..text.len() - clean_label.len() - 1].into();
      lexer.tokens.push(Token::new(TokenType::NowDocOpen, clean_label.clone(), snapshot));
      lexer.tokens.push(Token::new(TokenType::EncapsedString, text, &content_snapshot));
      lexer.tokens.push(
        Token::new(TokenType::NowDocClose, clean_label, lexer.control.get_last_snapshot())
      );
      Ok(())
    } else {
      lexer.tokens.push(Token::new(TokenType::HeredocOpen, label.clone(), snapshot));
      Self::get_parts(lexer, &label, SeriesCheckerMode::Heredoc)?;
      lexer.tokens.push(
        Token::new(TokenType::HeredocClose, label, lexer.control.get_last_snapshot())
      );
      Ok(())
    }
  }
}
