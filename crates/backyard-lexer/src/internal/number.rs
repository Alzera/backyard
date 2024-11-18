use crate::error::LexResult;
use crate::utils::get_char_until;
use crate::token::{ Token, TokenType };

pub struct NumberToken;

impl NumberToken {
  pub fn lex(chars: &Vec<char>, position: &mut usize) -> LexResult {
    if let Some(&next) = chars.get(*position) {
      if next == 'x' {
        *position += 1;
        let t = get_char_until(&chars, position, |ch, _| !ch.is_alphanumeric());
        let mut n = "0x".to_string();
        n.push_str(&t);
        return Ok(vec![Token::new(TokenType::NumberHex, n)]);
      }
    }
    *position -= 1;
    let t = get_char_until(&chars, position, |ch, _| !(ch.is_digit(10) || *ch == '.'));
    Ok(vec![Token::new(TokenType::Number, t.to_string())])
  }
}
