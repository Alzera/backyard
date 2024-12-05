use backyard_lexer::token::{ Token, TokenType };

#[derive(Debug, Clone)]
pub enum Lookup<'a> {
  Equal(&'a [TokenType]),
  Optional(&'a [TokenType]),
  Any,
  OptionalType,
  Modifiers(&'a [&'a [TokenType]]),
}

#[derive(Debug, Clone)]
pub struct LookupResult {
  pub size: usize,
  pub wrapper: LookupResultWrapper,
}

impl LookupResult {
  pub fn is_empty(&self) -> bool {
    self.size == 0
  }
}

#[derive(Debug, Clone)]
pub enum LookupResultWrapper {
  Equal(Token),
  Optional(Option<Token>),
  Any(Token),
  OptionalType(OptionalTypeResult),
  Modifier(Vec<Option<Token>>),
}

#[derive(Debug, Clone)]
pub enum OptionalTypeResult {
  None,
  Single(Token),
  Nullable(Token, Token),
  Union(Vec<OptionalTypeResult>),
  Intersection(Vec<OptionalTypeResult>),
}
const TYPES: [TokenType; 11] = [
  TokenType::Identifier,
  TokenType::Name,
  TokenType::Type,
  TokenType::Callable,
  TokenType::Static,
  TokenType::SelfKeyword,
  TokenType::Array,
  TokenType::True,
  TokenType::False,
  TokenType::Null,
  TokenType::Parent,
];

pub fn match_pattern(tokens: &[Token], pattern: &[Lookup]) -> Option<Vec<LookupResult>> {
  let mut result: Vec<LookupResult> = Vec::new();
  let mut check_position = 0;

  for p in pattern.iter() {
    match p {
      Lookup::Equal(contains_tokens) => {
        let cur = tokens.get(check_position);
        check_position += 1;
        cur?;
        let current_token = cur.unwrap();
        result.push(LookupResult {
          size: 1,
          wrapper: LookupResultWrapper::Equal(current_token.to_owned()),
        });
        if !contains_tokens.contains(&current_token.token_type) {
          return None;
        }
      }
      Lookup::Optional(contains_tokens) => {
        let cur = tokens.get(check_position);
        if cur.is_none() {
          result.push(LookupResult {
            size: 0,
            wrapper: LookupResultWrapper::Optional(None),
          });
          continue;
        }
        let current_token = cur.unwrap();
        if contains_tokens.contains(&current_token.token_type) {
          result.push(LookupResult {
            size: 1,
            wrapper: LookupResultWrapper::Optional(Some(current_token.to_owned())),
          });
          check_position += 1;
        } else {
          result.push(LookupResult {
            size: 0,
            wrapper: LookupResultWrapper::Optional(None),
          });
        }
      }
      Lookup::Any => {
        let cur = tokens.get(check_position);
        check_position += 1;
        cur?;
        let current_token = cur.unwrap();
        result.push(LookupResult {
          size: 1,
          wrapper: LookupResultWrapper::Any(current_token.to_owned()),
        });
      }
      Lookup::OptionalType => {
        let cur = tokens.get(check_position);
        if cur.is_none() {
          result.push(LookupResult {
            size: 0,
            wrapper: LookupResultWrapper::OptionalType(OptionalTypeResult::None),
          });
          continue;
        }
        let cur = cur.unwrap();
        let next = tokens.get(check_position + 1);
        if cur.token_type == TokenType::QuestionMark {
          if let Some(next) = next {
            if TYPES.contains(&next.token_type) {
              check_position += 2;
              result.push(LookupResult {
                size: 2,
                wrapper: LookupResultWrapper::OptionalType(
                  OptionalTypeResult::Nullable(cur.to_owned(), next.to_owned())
                ),
              });
              continue;
            }
          }
          result.push(LookupResult {
            size: 0,
            wrapper: LookupResultWrapper::OptionalType(OptionalTypeResult::None),
          });
          continue;
        }
        let old_check_position = check_position;
        let parsed = parse_type(tokens, &mut check_position);
        result.push(LookupResult {
          size: check_position - old_check_position,
          wrapper: LookupResultWrapper::OptionalType(parsed),
        });
      }
      Lookup::Modifiers(modifiers_rule) => {
        let mut modifiers: Vec<Option<Token>> = modifiers_rule
          .iter()
          .map(|_| None)
          .collect();
        let mut pos = 0;
        loop {
          let token = tokens.get(check_position + pos);
          token?;
          pos += 1;
          if pos > modifiers_rule.len() {
            break;
          }
          let mut assigned = false;
          let token = token.unwrap();
          for (i, modifier) in modifiers_rule.iter().enumerate() {
            if modifiers[i].is_some() {
              continue;
            }
            if modifier.contains(&token.token_type) {
              modifiers[i] = Some(token.to_owned());
              assigned = true;
              break;
            }
          }
          if !assigned {
            break;
          }
        }
        if pos != 0 {
          pos -= 1;
          check_position += pos;
        }
        result.push(LookupResult {
          size: pos,
          wrapper: LookupResultWrapper::Modifier(modifiers),
        });
      }
    }
  }
  if result.len() != pattern.len() {
    panic!("match_pattern fail");
  }
  Some(result)
}

fn parse_type(tokens: &[Token], index: &mut usize) -> OptionalTypeResult {
  let token = if let Some(t) = tokens.get(*index) {
    t
  } else {
    return OptionalTypeResult::None;
  };
  if token.token_type == TokenType::LeftParenthesis {
    *index += 1;
    let child = parse_type(tokens, index);
    let token = if let Some(t) = tokens.get(*index) {
      t
    } else {
      return OptionalTypeResult::None;
    };
    if token.token_type == TokenType::BitwiseAnd {
      *index += 1;
      if
        let Some(mut next) = parse_union_or_intersection_type(tokens, index, TokenType::BitwiseAnd)
      {
        next.insert(0, child);
        return OptionalTypeResult::Intersection(next);
      } else {
        return child;
      }
    } else if token.token_type == TokenType::BitwiseOr {
      *index += 1;
      if let Some(mut next) = parse_union_or_intersection_type(tokens, index, TokenType::BitwiseOr) {
        next.insert(0, child);
        return OptionalTypeResult::Union(next);
      } else {
        return child;
      }
    } else if token.token_type == TokenType::RightParenthesis {
      *index += 1;
      return child;
    }
  } else if TYPES.contains(&token.token_type) {
    let next_token = if let Some(t) = tokens.get(*index + 1) {
      t
    } else {
      return OptionalTypeResult::None;
    };
    if next_token.token_type == TokenType::BitwiseAnd {
      if let Some(child) = parse_union_or_intersection_type(tokens, index, TokenType::BitwiseAnd) {
        return OptionalTypeResult::Intersection(child);
      }
    } else if next_token.token_type == TokenType::BitwiseOr {
      if let Some(child) = parse_union_or_intersection_type(tokens, index, TokenType::BitwiseOr) {
        return OptionalTypeResult::Union(child);
      }
    }
    *index += 1;
    return OptionalTypeResult::Single(token.to_owned());
  }
  OptionalTypeResult::None
}

fn parse_union_or_intersection_type(
  tokens: &[Token],
  index: &mut usize,
  separator: TokenType
) -> Option<Vec<OptionalTypeResult>> {
  let mut result: Vec<OptionalTypeResult> = vec![];
  let mut last_token_type = None;
  loop {
    let token = tokens.get(*index)?;
    if
      !TYPES.contains(&token.token_type) &&
      ![separator, TokenType::LeftParenthesis, TokenType::RightParenthesis].contains(
        &token.token_type
      )
    {
      break;
    }
    *index += 1;
    if
      (last_token_type.is_none() || last_token_type.unwrap() == separator) &&
      TYPES.contains(&token.token_type)
    {
      last_token_type = Some(token.token_type);
      result.push(OptionalTypeResult::Single(token.to_owned()));
      continue;
    }
    if last_token_type.is_some() {
      if last_token_type.unwrap() == separator {
        if token.token_type == TokenType::LeftParenthesis {
          result.push(parse_type(tokens, index));
        } else if token.token_type == TokenType::RightParenthesis {
          break;
        }
      } else if token.token_type == separator {
        last_token_type = Some(token.token_type);
        continue;
      }
    }
    break;
  }
  if let Some(m) = last_token_type {
    if [TokenType::BitwiseAnd, TokenType::BitwiseOr].contains(&m) {
      *index -= 1;
    }
  }
  if result.is_empty() {
    return None;
  }
  Some(result)
}
