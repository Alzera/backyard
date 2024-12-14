use bstr::BString;
use bumpalo::{ collections::Vec, Bump };
use backyard_lexer::token::{ Token, TokenType };
use backyard_nodes::{
  IntersectionTypeNode,
  Node,
  NodeType,
  RangeLocation,
  TypeNode,
  UnionTypeNode,
  Visibility,
  utils::CloneIn,
};

use crate::{ error::ParserError, parser::{ LocationHelper, Parser } };

#[derive(Debug, Clone)]
pub enum Lookup<'a> {
  Equal(&'a [TokenType]),
  Optional(&'a [TokenType]),
  Any,
  OptionalType,
  Modifiers(&'a [ModifierLookup<'a>]),
}

#[derive(Debug, Clone)]
pub enum ModifierLookup<'a> {
  Visibility,
  Custom(&'a [TokenType]),
}

#[derive(Debug)]
pub struct LookupResult<'a> {
  pub size: usize,
  pub wrapper: LookupResultWrapper<'a>,
}

impl<'a> LookupResult<'a> {
  pub fn is_empty(&self) -> bool {
    self.size == 0
  }

  pub fn as_equal(&self) -> Result<&Token, ParserError> {
    if let LookupResultWrapper::Equal(v) = &self.wrapper {
      Ok(v)
    } else {
      Err(ParserError::Internal)
    }
  }

  pub fn as_optional(&self) -> Option<&Token> {
    if let LookupResultWrapper::Optional(Some(v)) = &self.wrapper { Some(v) } else { None }
  }

  pub fn as_optional_type<'arena>(&self, arena: &'arena Bump) -> Option<Node<'arena>> {
    if let LookupResultWrapper::OptionalType(Some(v)) = &self.wrapper {
      Some(v.clone_in(arena))
    } else {
      None
    }
  }

  pub fn as_modifier(&self) -> Option<&[ModifierResult]> {
    if let LookupResultWrapper::Modifier(v) = &self.wrapper { Some(v.as_slice()) } else { None }
  }
}

#[derive(Debug)]
pub enum LookupResultWrapper<'a> {
  Equal(Token),
  Optional(Option<Token>),
  Any(Token),
  OptionalType(Option<Node<'a>>),
  Modifier(std::vec::Vec<ModifierResult>),
}

#[derive(Debug)]
pub enum ModifierResult {
  Visibility(std::vec::Vec<Token>),
  Custom(Option<Token>),
}

impl ModifierResult {
  pub fn as_visibilities(&self) -> std::vec::Vec<Visibility> {
    if let ModifierResult::Visibility(v) = self {
      v.iter()
        .filter_map(|x| Visibility::try_from(&x.value).ok())
        .collect()
    } else {
      vec![]
    }
  }

  pub fn as_custom<T, C>(&self, callback: C) -> Option<T>
    where C: FnOnce(&BString) -> Result<T, String>
  {
    if let ModifierResult::Custom(Some(v)) = self { callback(&v.value).ok() } else { None }
  }
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

pub fn match_pattern<'arena, 'b>(
  parser: &Parser<'arena, 'b>,
  tokens: &[Token],
  pattern: &[Lookup]
) -> Option<std::vec::Vec<LookupResult<'arena>>> {
  let mut result = std::vec::Vec::with_capacity(pattern.len());
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
            wrapper: LookupResultWrapper::OptionalType(None),
          });
          continue;
        }
        let cur = cur.unwrap();
        let next = tokens.get(check_position + 1);
        if cur.token_type == TokenType::QuestionMark {
          if let Some(next) = next {
            if TYPES.contains(&next.token_type) {
              check_position += 2;
              let start_loc = cur.get_location().unwrap();
              let end_loc = next.get_range_location().unwrap().end;
              result.push(LookupResult {
                size: 2,
                wrapper: LookupResultWrapper::OptionalType(
                  Some(
                    TypeNode::loc(
                      true,
                      next.value.to_owned(),
                      Some(RangeLocation {
                        start: start_loc,
                        end: end_loc,
                      })
                    )
                  )
                ),
              });
              continue;
            }
          }
          result.push(LookupResult {
            size: 0,
            wrapper: LookupResultWrapper::OptionalType(None),
          });
          continue;
        }
        let old_check_position = check_position;
        let mut parsed = parse_type(parser, tokens, &mut check_position);
        if let Some(to_check) = &parsed {
          if to_check.node_type == NodeType::Type && cur.token_type == TokenType::Identifier {
            if let Some(next) = next {
              if next.token_type == TokenType::Assignment {
                parsed = None;
                check_position = old_check_position;
              }
            }
          }
        }
        result.push(LookupResult {
          size: check_position - old_check_position,
          wrapper: LookupResultWrapper::OptionalType(parsed),
        });
      }
      Lookup::Modifiers(modifiers_rule) => {
        let mut modifiers: std::vec::Vec<std::vec::Vec<Token>> = modifiers_rule
          .iter()
          .map(|_| vec![])
          .collect();
        let mut pos = 0;
        loop {
          let token = tokens.get(check_position + pos);
          token?;
          pos += 1;
          let mut assigned = false;
          let token = token.unwrap();
          for (i, modifier) in modifiers_rule.iter().enumerate() {
            if let ModifierLookup::Custom(types) = modifier {
              if !modifiers[i].is_empty() {
                continue;
              }
              if types.contains(&token.token_type) {
                modifiers[i].push(token.to_owned());
                assigned = true;
                break;
              }
            } else if
              [
                TokenType::Private,
                TokenType::PrivateGet,
                TokenType::PrivateSet,
                TokenType::Protected,
                TokenType::ProtectedGet,
                TokenType::ProtectedSet,
                TokenType::Public,
                TokenType::PublicGet,
                TokenType::PublicSet,
              ].contains(&token.token_type)
            {
              modifiers[i].push(token.to_owned());
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
        let modifiers = modifiers
          .iter()
          .enumerate()
          .map(|(i, x)| {
            if let ModifierLookup::Visibility = modifiers_rule[i] {
              ModifierResult::Visibility(x.to_owned())
            } else {
              ModifierResult::Custom(x.first().cloned())
            }
          })
          .collect();
        result.push(LookupResult {
          size: pos,
          wrapper: LookupResultWrapper::Modifier(modifiers),
        });
      }
    }
  }
  Some(result)
}

fn get_range_location_from_vec_node(vec_node: &[Node]) -> Option<RangeLocation> {
  if
    let Some(start_loc) = vec_node
      .first()
      .map(|x| x.loc.as_ref().map(|loc| loc.start.clone()))
      .unwrap_or_default()
  {
    vec_node
      .last()
      .map(|x| x.loc.as_ref().map(|loc| loc.end.clone()))
      .unwrap_or_default()
      .map(|end_loc| RangeLocation { start: start_loc, end: end_loc })
  } else {
    None
  }
}

fn parse_type<'arena, 'a>(
  parser: &Parser<'arena, 'a>,
  tokens: &[Token],
  index: &mut usize
) -> Option<Node<'arena>> {
  let token = tokens.get(*index)?;
  if token.token_type == TokenType::LeftParenthesis {
    *index += 1;
    let child = parse_type(parser, tokens, index)?;
    let token = tokens.get(*index)?;
    if token.token_type == TokenType::BitwiseAnd {
      *index += 1;
      if
        let Some(mut next) = parse_union_or_intersection_type(
          parser,
          tokens,
          index,
          TokenType::BitwiseAnd
        )
      {
        next.insert(0, child);
        let loc = get_range_location_from_vec_node(&next);
        return Some(IntersectionTypeNode::loc(next, loc));
      } else {
        return Some(child);
      }
    } else if token.token_type == TokenType::BitwiseOr {
      *index += 1;
      if
        let Some(mut next) = parse_union_or_intersection_type(
          parser,
          tokens,
          index,
          TokenType::BitwiseOr
        )
      {
        next.insert(0, child);
        let loc = get_range_location_from_vec_node(&next);
        return Some(UnionTypeNode::loc(next, loc));
      } else {
        return Some(child);
      }
    } else if token.token_type == TokenType::RightParenthesis {
      *index += 1;
      return Some(child);
    }
  } else if TYPES.contains(&token.token_type) {
    let next_token = tokens.get(*index + 1)?;
    if next_token.token_type == TokenType::BitwiseAnd {
      if
        let Some(child) = parse_union_or_intersection_type(
          parser,
          tokens,
          index,
          TokenType::BitwiseAnd
        )
      {
        let loc = get_range_location_from_vec_node(&child);
        return Some(IntersectionTypeNode::loc(child, loc));
      }
    } else if next_token.token_type == TokenType::BitwiseOr {
      if
        let Some(child) = parse_union_or_intersection_type(
          parser,
          tokens,
          index,
          TokenType::BitwiseOr
        )
      {
        let loc = get_range_location_from_vec_node(&child);
        return Some(UnionTypeNode::loc(child, loc));
      }
    }
    *index += 1;
    let loc = token.get_range_location();
    return Some(TypeNode::loc(false, token.value.to_owned(), loc));
  }
  None
}

fn parse_union_or_intersection_type<'arena, 'b>(
  parser: &Parser<'arena, 'b>,
  tokens: &[Token],
  index: &mut usize,
  separator: TokenType
) -> Option<Vec<'arena, Node<'arena>>> {
  let mut result = bumpalo::vec![in parser.arena];
  let mut last_token_type = None;
  loop {
    let token = tokens.get(*index)?;
    if
      TYPES.contains(&token.token_type) ||
      [separator, TokenType::LeftParenthesis, TokenType::RightParenthesis].contains(
        &token.token_type
      )
    {
      *index += 1;
      if last_token_type.is_none() || last_token_type.unwrap() == separator {
        if TYPES.contains(&token.token_type) {
          last_token_type = Some(token.token_type);
          let loc = token.get_range_location();
          result.push(TypeNode::loc(false, token.value.to_owned(), loc));
          continue;
        } else if token.token_type == TokenType::LeftParenthesis {
          result.push(parse_type(parser, tokens, index)?);
        }
      } else if last_token_type.is_some() && last_token_type.unwrap() != separator {
        if token.token_type == separator {
          last_token_type = Some(token.token_type);
          continue;
        } else if token.token_type == TokenType::RightParenthesis {
          break;
        } else {
          *index -= 1;
        }
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
