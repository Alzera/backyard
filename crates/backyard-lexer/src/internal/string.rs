use utils::guard;

use crate::internal::variable::VariableToken;
use crate::error::LexResult;
use crate::lexer::{ ControlSnapshot, Lexer, SeriesChecker };
use crate::token::{ Token, TokenType };

pub struct StringToken;

impl StringToken {
  fn get_parts(lexer: &mut Lexer, result: &mut Vec<Token>, breaker: &str) {
    let mut checker = SeriesChecker::new(&[breaker]);
    let mut need_check_condition: Vec<char> = breaker.chars().collect();
    need_check_condition.push('$');
    need_check_condition.push('{');
    loop {
      let mut snapshot = None;
      let mut t = lexer.control.next_char_until(|control, ch, end_position| {
        if snapshot.is_none() {
          snapshot = Some(control.get_snapshot());
        }
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
                return true;
              }
            }
          }
        }
        false
      });

      let current = guard!(lexer.control.peek_char(None), {
        break;
      });
      if checker.check().is_some() {
        t.push(current);
        t = t[..t.len() - breaker.len()].to_string();
        result.push(Token::new(TokenType::EncapsedString, t, &snapshot.unwrap()));
        lexer.control.next_char();
        break;
      }
      if !t.is_empty() {
        result.push(Token::new(TokenType::EncapsedString, t, &snapshot.unwrap()));
      }

      let next = lexer.control.peek_char(Some(lexer.control.get_position() + 1));
      let snapshot = lexer.control.get_snapshot();

      if current == '$' {
        if next.is_some() && next.unwrap() == '{' {
          let tokens = lexer.next_tokens_until_right_bracket();
          result.extend(tokens);
        } else {
          lexer.control.next_char();
          if let Ok(token) = VariableToken::lex(lexer, &snapshot) {
            result.extend(token);
          }
        }
      } else if next.is_some() && current == '{' && next.unwrap() == '$' {
        lexer.control.next_char();
        let tokens = lexer.next_tokens_until_right_bracket();
        result.push(Token::new(TokenType::AdvanceInterpolationOpen, "{", &snapshot));
        result.extend(tokens);
        result.push(
          Token::new(TokenType::AdvanceInterpolationClose, "}", &lexer.control.get_snapshot())
        );
      }
    }
  }

  pub fn lex(lexer: &mut Lexer, breaker: char, snapshot: &ControlSnapshot) -> LexResult {
    let mut result = vec![
      Token::new(TokenType::EncapsedStringOpen, String::from(breaker), snapshot)
    ];

    Self::get_parts(lexer, &mut result, &breaker.to_string());

    if result.len() < 3 {
      let mut t = result.get(1).unwrap().to_owned();
      t.token_type = TokenType::String;
      t.value = format!("{}{}{}", breaker, t.value, breaker);
      return Ok(vec![t]);
    }

    result.push(
      Token::new(
        TokenType::EncapsedStringClose,
        String::from(breaker),
        &lexer.control.get_snapshot()
      )
    );

    Ok(result)
  }

  pub fn lex_doc(lexer: &mut Lexer, snapshot: &ControlSnapshot) -> LexResult {
    let label = lexer.control.next_char_until(|_, i, _| *i == '\n');
    if
      !label
        .trim()
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
    if label.starts_with("'") && label.ends_with("'") {
      let clean_label = label
        .get(1..label.len() - 1)
        .unwrap_or_default()
        .to_string();
      let mut checker = SeriesChecker::new(&[&clean_label]);
      let mut should_break = false;
      let mut content_snapshot = None;
      let text = lexer.control.next_char_until(|control, i, _| {
        if content_snapshot.is_none() {
          content_snapshot = Some(control.get_snapshot());
        }
        checker.push(*i);
        let t = should_break;
        should_break = checker.check().is_some();
        t
      });
      let text = text[..text.len() - clean_label.len()].to_string();
      Ok(
        vec![
          Token::new(TokenType::NowDocOpen, &clean_label, snapshot),
          Token::new(TokenType::EncapsedString, text, &content_snapshot.unwrap()),
          Token::new(TokenType::NowDocClose, &clean_label, &lexer.control.get_snapshot())
        ]
      )
    } else {
      let mut result = vec![Token::new(TokenType::HeredocOpen, &label, snapshot)];
      Self::get_parts(lexer, &mut result, &label);
      result.push(Token::new(TokenType::HeredocClose, &label, &lexer.control.get_snapshot()));
      Ok(result)
    }
  }
}
