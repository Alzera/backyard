use utils::guard;

use crate::internal::variable::VariableToken;
use crate::error::LexResult;
use crate::lexer::Lexer;
use crate::token::{ Token, TokenType };

pub struct StringToken;

impl StringToken {
  pub fn lex(lexer: &mut Lexer, breaker: char) -> LexResult {
    let mut result: Vec<Token> = vec![
      Token::new(TokenType::EncapsedStringOpen, String::from(breaker))
    ];

    loop {
      let mut ignore_last_escape = false;
      let mut last_char = None;
      let t = lexer.control.next_char_until(|control, ch, end_position| {
        let is_escaped = last_char.is_some() && last_char.unwrap() == '\\' && !ignore_last_escape;
        ignore_last_escape = is_escaped;
        last_char = Some(*ch);

        if [breaker, '$', '{'].contains(ch) {
          if *ch == breaker && !is_escaped {
            return true;
          } else if *ch == '$' {
            if is_escaped {
              return false;
            }
            if let Some(next) = control.peek_char(Some(*end_position + 1)) {
              if next == '_' || next.is_alphabetic() {
                return true;
              }
            }
          } else if *ch == '{' {
            if is_escaped {
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

      if !t.is_empty() {
        result.push(Token::new(TokenType::EncapsedString, t));
      }

      let current = guard!(lexer.control.peek_char(None), {
        break;
      });
      if current == breaker {
        lexer.control.next_char();
        break;
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

    if result.len() < 3 {
      let t = match result.get(1) {
        Some(t) => t.value.clone(),
        _ => String::from(""),
      };
      return Ok(vec![Token::new(TokenType::String, t)]);
    }

    result.push(Token::new(TokenType::EncapsedStringClose, String::from(breaker)));

    Ok(result)
  }
}
