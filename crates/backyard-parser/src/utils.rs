use backyard_lexer::token::{ Token, TokenType };

#[derive(Debug, Clone)]
pub enum Lookup<'a> {
  Equal(&'a [TokenType]),
  Optional(&'a [TokenType]),
  Any,
}

pub fn match_pattern(tokens: &[Token], pattern: &[Lookup]) -> Option<Vec<Vec<Token>>> {
  let mut result: Vec<Vec<Token>> = Vec::new();
  let mut check_position = 0;

  for p in pattern.iter() {
    match p {
      Lookup::Equal(contains_tokens) => {
        let cur = tokens.get(check_position);
        check_position += 1;
        cur?;
        let current_token = cur.unwrap();
        result.push(vec![current_token.to_owned()]);
        if !contains_tokens.contains(&current_token.token_type) {
          return None;
        }
      }
      Lookup::Any => {
        let cur = tokens.get(check_position);
        check_position += 1;
        cur?;
        let current_token = cur.unwrap();
        result.push(vec![current_token.to_owned()]);
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
