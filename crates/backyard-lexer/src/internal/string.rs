use utils::guard;

use crate::internal::variable::VariableToken;
use crate::error::LexResult;
use crate::lexer::{ Lexer, SeriesChecker };
use crate::token::{ Token, TokenType };

pub struct StringToken;

impl StringToken {
  fn get_parts(lexer: &mut Lexer, result: &mut Vec<Token>, breaker: &str) {
    let mut checker = SeriesChecker::new(&[breaker]);
    let mut need_check_condition: Vec<char> = breaker.chars().collect();
    need_check_condition.push('$');
    need_check_condition.push('{');
    loop {
      let mut t = lexer.control.next_char_until(|control, ch, end_position| {
        checker.push(ch.clone());
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
        result.push(Token::new(TokenType::EncapsedString, t));
        lexer.control.next_char();
        break;
      }
      if !t.is_empty() {
        result.push(Token::new(TokenType::EncapsedString, t));
      }

      let next = lexer.control.peek_char(Some(lexer.control.get_position() + 1));

      if current == '$' {
        if next.is_some() && next.unwrap() == '{' {
          let tokens = lexer.next_tokens_until_right_bracket();
          result.extend(tokens);
        } else {
          lexer.control.next_char();
          if let Ok(token) = VariableToken::lex(lexer) {
            result.extend(token);
          }
        }
      } else if next.is_some() && current == '{' && next.unwrap() == '$' {
        lexer.control.next_char();
        let tokens = lexer.next_tokens_until_right_bracket();
        result.push(Token::new(TokenType::AdvanceInterpolationOpen, "{"));
        result.extend(tokens);
        result.push(Token::new(TokenType::AdvanceInterpolationClose, "}"));
      }
    }
  }

  pub fn lex(lexer: &mut Lexer, breaker: char) -> LexResult {
    let mut result = vec![Token::new(TokenType::EncapsedStringOpen, String::from(breaker))];

    Self::get_parts(lexer, &mut result, &breaker.to_string());

    if result.len() < 3 {
      let t = match result.get(1) {
        Some(t) => t.value.to_owned(),
        _ => String::from(""),
      };
      return Ok(vec![Token::new(TokenType::String, format!("{}{}{}", breaker, t, breaker))]);
    }

    result.push(Token::new(TokenType::EncapsedStringClose, String::from(breaker)));

    Ok(result)
  }

  pub fn lex_doc(lexer: &mut Lexer) -> LexResult {
    let label = lexer.control.next_char_until(|_, i, _| *i == '\n');
    if
      !label
        .chars()
        .enumerate()
        .fold(
          true,
          |acc, (i, ch)|
            acc &&
            (ch.is_alphanumeric() || ch == '_' || ((i == 0 || i == label.len() - 1) && ch == '\''))
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
      let text = lexer.control.next_char_until(|_, i, _| {
        checker.push(i.clone());
        let t = should_break.clone();
        should_break = checker.check().is_some();
        t
      });
      let text = text[..text.len() - clean_label.len()].to_string();
      Ok(
        vec![
          Token::new(TokenType::NowDocOpen, &clean_label),
          Token::new(TokenType::EncapsedString, text),
          Token::new(TokenType::NowDocClose, &clean_label)
        ]
      )
    } else {
      let mut result = vec![Token::new(TokenType::HeredocOpen, &label)];
      Self::get_parts(lexer, &mut result, &label);
      result.push(Token::new(TokenType::HeredocClose, &label));
      Ok(result)
    }
  }
}
