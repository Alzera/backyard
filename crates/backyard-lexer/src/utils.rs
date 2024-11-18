use crate::{ lexer::Lexer, token::{ Token, TokenType } };

pub fn get_char_until<F>(chars: &Vec<char>, position: &mut usize, mut until: F) -> String
  where F: FnMut(&char, &mut usize) -> bool
{
  let start_position = *position;
  let mut end_position = *position;
  while let Some(ch) = chars.get(end_position) {
    if until(ch, &mut end_position) {
      break;
    }
    end_position += 1;
  }

  let result: Vec<char> = chars[start_position..end_position].to_vec();
  *position = end_position;
  result.iter().collect()
}

pub fn get_tokens_until_right_bracket(lexer: &mut Lexer) -> Vec<Token> {
  get_tokens_level(
    lexer,
    1,
    [TokenType::LeftCurlyBracket].to_vec(),
    [TokenType::RightCurlyBracket].to_vec()
  )
}

pub fn get_tokens_level(
  lexer: &mut Lexer,
  start_level: usize,
  level_ups: Vec<TokenType>,
  level_downs: Vec<TokenType>
) -> Vec<Token> {
  let mut result: Vec<Token> = Vec::new();
  let mut level = start_level;
  loop {
    if let Ok(tokens) = lexer.next_tokens(true) {
      if let Some(token) = tokens.first() {
        if level_ups.contains(&token.token_type) {
          level += 1;
        } else if level_downs.contains(&token.token_type) {
          level -= 1;
          if level == 0 {
            break result;
          }
        }
      }
      result.extend(tokens);
    } else {
      break result;
    }
  }
}
