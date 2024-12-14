use bstr::{ BString, ByteSlice, ByteVec };

use crate::error::{ LexError, LexResult };
use crate::internal::variable::VariableToken;
use crate::lexer::{ ControlSnapshot, Lexer, SeriesChecker, SeriesCheckerMode, U8Ext };
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
    let mut need_check_condition: Vec<u8> = breaker.as_bytes().to_vec();
    need_check_condition.push(b'$');
    need_check_condition.push(b'{');
    loop {
      let snapshot = lexer.control.get_snapshot();
      let mut t = lexer.control.next_char_until(0, |control, ch, end_position| {
        checker.push(ch);
        if need_check_condition.contains(&ch) {
          if checker.check().is_some() {
            return true;
          } else if ch == b'$' {
            if checker.is_last_escaped() {
              return false;
            }
            if let Some(next) = control.peek_char(Some(*end_position + 1)) {
              if *next == b'_' || next.is_alphabetic() {
                return true;
              }
            }
          } else if ch == b'{' {
            if checker.is_last_escaped() {
              return false;
            }
            if let Some(next) = control.peek_char(Some(*end_position + 1)) {
              if *next == b'$' {
                if let Some(next) = control.peek_char(Some(*end_position + 2)) {
                  if *next == b'_' || *next == b'{' || next.is_alphabetic() {
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
        current.to_owned()
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
      let next = lexer.control
        .peek_char(Some(lexer.control.get_position() + 1))
        .map(|x| x.to_owned());
      let snapshot = lexer.control.get_snapshot();

      if current == b'$' {
        lexer.control.next_char();
        VariableToken::lex(lexer, &snapshot)?;
      } else if next.is_some() && current == b'{' && next.unwrap() == b'$' {
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
    let mut text = lexer.control.next_char_until(0, |_, i, _| {
      checker.push(i);
      checker.check().is_some()
    });
    lexer.control.next_char();
    text.insert_str(0, breaker);
    text.push_str(breaker);
    lexer.tokens.push(Token::new(TokenType::String, text, snapshot));
    Ok(())
  }

  pub fn lex(lexer: &mut Lexer, breaker: &str, snapshot: &ControlSnapshot) -> LexResult {
    lexer.tokens.push(Token::new(TokenType::EncapsedStringOpen, breaker.into(), snapshot));

    let is_without_encapsed = Self::get_parts(lexer, &breaker, SeriesCheckerMode::String)?;

    if is_without_encapsed {
      if let Some(string_token) = lexer.tokens.pop() {
        lexer.tokens.pop();
        let mut value = string_token.value;
        value.insert_str(0, breaker);
        value.push_str(breaker);
        lexer.tokens.push(Token::new(TokenType::String, value, snapshot));
        return Ok(());
      }
    }

    lexer.tokens.push(
      Token::new(TokenType::EncapsedStringClose, breaker.into(), lexer.control.get_last_snapshot())
    );

    Ok(())
  }

  pub fn lex_doc(lexer: &mut Lexer, snapshot: &ControlSnapshot) -> LexResult {
    let label = lexer.control.next_char_until(0, |_, i, _| i == b'\n');
    let label = BString::new(label.trim().to_vec());
    if
      !label
        .iter()
        .enumerate()
        .fold(true, |acc, (i, ch)| {
          acc &&
            (ch.is_alphanumeric() ||
              *ch == b'_' ||
              ((i == 0 || i == label.len() - 1) && [b'\'', b'"'].contains(ch)))
        })
    {
      return Err(lexer.control.error_unrecognized(label.to_string().split_off(1)));
    }
    if label.starts_with(&[b'\'']) && label.ends_with(&[b'\'']) {
      let clean_label: BString = label
        .get(1..label.len() - 1)
        .unwrap_or_default()
        .into();
      let clean_label_str = clean_label.to_string();
      let againsts: [&str; 1] = [&clean_label_str];
      let mut checker = SeriesChecker::new(&againsts, SeriesCheckerMode::Heredoc);
      let mut should_break = false;
      let content_snapshot = lexer.control.get_snapshot();
      let text = lexer.control.next_char_until(0, |_, i, _| {
        checker.push(i);
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
      let label = BString::new(label.to_vec());
      lexer.tokens.push(Token::new(TokenType::HeredocOpen, label.clone(), snapshot));
      Self::get_parts(lexer, &label.to_string(), SeriesCheckerMode::Heredoc)?;
      lexer.tokens.push(
        Token::new(TokenType::HeredocClose, label, lexer.control.get_last_snapshot())
      );
      Ok(())
    }
  }
}
