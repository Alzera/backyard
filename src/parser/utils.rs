use crate::lexer::token::{ Token, TokenType };

#[derive(Debug, Clone)]
pub enum Lookup {
  Equal(Vec<TokenType>),
  Optional(Vec<TokenType>),
}

pub fn match_pattern(tokens: &Vec<Token>, pattern: Vec<Lookup>) -> Option<Vec<Vec<Token>>> {
  let mut result: Vec<Vec<Token>> = Vec::new();
  let mut check_position = 0;

  // println!("tokens: {:?}", tokens);

  for p in pattern.iter() {
    match p {
      Lookup::Equal(contains_tokens) => {
        let cur = tokens.get(check_position);
        check_position += 1;
        if cur.is_none() {
          //          println!("Contains test exhausted: {:?}", result);
          return None;
        }
        let current_token = cur.unwrap();
        result.push(vec![current_token.to_owned()]);
        if !contains_tokens.contains(&current_token.token_type) {
          //          println!("Contains fail: {:?}", p);
          return None;
        }
      }
      Lookup::Optional(contains_tokens) => {
        let cur = tokens.get(check_position);
        if cur.is_none() {
          result.push(vec![]);
          continue;
        }
        let current_token = cur.unwrap();
        if contains_tokens.contains(&current_token.token_type) {
          result.push(vec![current_token.to_owned()]);
          check_position += 1;
        } else {
          result.push(vec![]);
        }
      }
    }
  }

  Some(result)
}

pub fn some_or_default<T, U, F>(opt: Option<T>, default: U, access: F) -> U where F: Fn(T) -> U {
  match opt {
    Some(value) => access(value),
    None => default,
  }
}
