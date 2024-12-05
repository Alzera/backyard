use crate::error::LexResult;
use crate::internal::variable::VariableToken;
use crate::lexer::{ ControlSnapshot, Lexer, SeriesChecker, SeriesCheckerMode };
use crate::token::{ Token, TokenType };

pub struct StringToken;

impl StringToken {
  fn get_parts(lexer: &mut Lexer, result: &mut Vec<Token>, breaker: &str, mode: SeriesCheckerMode) {
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
      let current = if let Some(current) = lexer.control.peek_char(None) {
        current
      } else {
        break;
      };
      if checker.check().is_some() {
        t.push(current);
        t = t[..t.len() - breaker.len()].to_string();
        result.push(Token::new(TokenType::EncapsedString, t, &snapshot));
        lexer.control.next_char();
        break;
      }
      if !t.is_empty() {
        result.push(Token::new(TokenType::EncapsedString, t, &snapshot));
      }

      let next = lexer.control.peek_char(Some(lexer.control.get_position() + 1));
      let snapshot = lexer.control.get_snapshot();

      if current == '$' {
        lexer.control.next_char();
        if let Ok(token) = VariableToken::lex(lexer, &snapshot) {
          result.extend(token);
        }
      } else if next.is_some() && current == '{' && next.unwrap() == '$' {
        lexer.control.next_char();
        result.push(Token::new(TokenType::AdvanceInterpolationOpen, "{", &snapshot));
        let tokens = lexer.next_tokens_until_right_bracket();
        result.extend(tokens);
        result.push(
          Token::new(TokenType::AdvanceInterpolationClose, "}", lexer.control.get_last_snapshot())
        );
      }
    }
  }

  pub fn lex_basic(lexer: &mut Lexer, breaker: char, snapshot: &ControlSnapshot) -> LexResult {
    let checker_breaker: [&str; 1] = [&breaker.to_string()];
    let mut checker = SeriesChecker::new(&checker_breaker, SeriesCheckerMode::String);
    let text = lexer.control.next_char_until(|_, i, _| {
      checker.push(*i);
      checker.check().is_some()
    });
    lexer.control.next_char();

    return Ok(
      vec![Token::new(TokenType::String, format!("{}{}{}", breaker, text, breaker), snapshot)]
    );
  }

  pub fn lex(lexer: &mut Lexer, breaker: char, snapshot: &ControlSnapshot) -> LexResult {
    let mut result = vec![
      Token::new(TokenType::EncapsedStringOpen, String::from(breaker), snapshot)
    ];

    Self::get_parts(lexer, &mut result, &breaker.to_string(), SeriesCheckerMode::String);

    if result.len() < 3 {
      let t = if let Some(t) = result.get(1) { t.value.to_owned() } else { String::from("") };
      return Ok(
        vec![Token::new(TokenType::String, format!("{}{}{}", breaker, t, breaker), snapshot)]
      );
    }

    result.push(
      Token::new(
        TokenType::EncapsedStringClose,
        String::from(breaker),
        lexer.control.get_last_snapshot()
      )
    );

    Ok(result)
  }

  pub fn lex_doc(lexer: &mut Lexer, snapshot: &ControlSnapshot) -> LexResult {
    let label = lexer.control
      .next_char_until(|_, i, _| *i == '\n')
      .trim()
      .to_owned();
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
      let clean_label = label.get(1..label.len() - 1).unwrap_or_default();
      let againsts = [clean_label];
      let mut checker = SeriesChecker::new(&againsts, SeriesCheckerMode::Heredoc);
      let mut should_break = false;
      let content_snapshot = lexer.control.get_snapshot();
      let text = lexer.control.next_char_until(|_, i, _| {
        checker.push(*i);
        let t = should_break;
        should_break = checker.check().is_some();
        t
      });
      let text = text[..text.len() - clean_label.len() - 1].to_string();
      Ok(
        vec![
          Token::new(TokenType::NowDocOpen, &clean_label, snapshot),
          Token::new(TokenType::EncapsedString, text, &content_snapshot),
          Token::new(TokenType::NowDocClose, &clean_label, lexer.control.get_last_snapshot())
        ]
      )
    } else {
      let mut result = vec![Token::new(TokenType::HeredocOpen, &label, snapshot)];
      Self::get_parts(lexer, &mut result, &label, SeriesCheckerMode::Heredoc);
      result.push(Token::new(TokenType::HeredocClose, &label, lexer.control.get_last_snapshot()));
      Ok(result)
    }
  }
}
