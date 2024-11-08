use crate::lexer::internal::variable::VariableToken;
use crate::lexer::lexer::Lexer;
use crate::lexer::utils::{ get_char_until, get_tokens_until_right_bracket };
use crate::lexer::token::{ Token, TokenType };

pub struct StringToken {}

impl StringToken {
  pub fn lex(lexer: &mut Lexer, breaker: char) -> Option<Vec<Token>> {
    let mut result: Vec<Token> = vec![
      Token::new(TokenType::EncapsedStringOpen, String::from(breaker))
    ];

    loop {
      let mut ignore_last_escape = false;
      let mut last_char = None;
      let t = get_char_until(&lexer.chars, &mut lexer.position, |ch, end_position| {
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
            if let Some(next) = lexer.chars.get(*end_position + 1) {
              if *next == '_' || next.is_alphabetic() {
                return true;
              }
            }
          } else if *ch == '{' {
            if is_escaped {
              return false;
            }
            if let Some(next) = lexer.chars.get(*end_position + 1) {
              if *next == '$' {
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

      let c = lexer.chars.get(lexer.position);
      if c.is_none() {
        break;
      }
      let current = c.unwrap();

      if *current == breaker {
        lexer.position += 1;
        break;
      }
      let next = lexer.chars.get(lexer.position + 1);

      if *current == '$' {
        if next.is_some() && *next.unwrap() == '{' {
          let tokens = get_tokens_until_right_bracket(lexer);
          result.extend(tokens);
        } else {
          lexer.position += 1;
          if let Some(token) = VariableToken::lex(lexer) {
            result.extend(token);
          }
        }
      } else if next.is_some() && *current == '{' && *next.unwrap() == '$' {
        lexer.position += 1;
        let tokens = get_tokens_until_right_bracket(lexer);
        result.push(Token::new(TokenType::AdvanceInterpolationOpen, "{"));
        result.extend(tokens);
        result.push(Token::new(TokenType::AdvanceInterpolationClose, "}"));
        // lexer.position += 1;
      }
    }

    if result.len() < 3 {
      let t = match result.get(1) {
        Some(t) => t.value.clone(),
        _ => String::from(""),
      };
      return Some(vec![Token::new(TokenType::String, t)]);
    }

    result.push(Token::new(TokenType::EncapsedStringClose, String::from(breaker)));

    Some(result)
  }
}
