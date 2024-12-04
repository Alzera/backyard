use backyard_lexer::token::Token;
use backyard_nodes::node::{
  IntersectionTypeNode,
  Location,
  Node,
  RangeLocation,
  TypeNode,
  UnionTypeNode,
};

use crate::{
  error::ParserError,
  parser::{ LoopArgument, Parser, LocationHelper },
  utils::{ match_pattern, Lookup, LookupResult, LookupResultWrapper, OptionalTypeResult },
};

#[derive(Debug, Clone)]
pub struct TypesParser;

impl TypesParser {
  #[allow(unused_assignments)]
  pub fn test(tokens: &[Token], _: &mut LoopArgument) -> Option<Vec<LookupResult>> {
    if let Some(m) = match_pattern(tokens, &[Lookup::OptionalType]) {
      if let Some(types) = m.first() {
        if !types.is_empty() {
          return Some(m);
        }
      }
    }
    None
  }

  pub fn parse(
    _: &mut Parser,
    matched: Vec<LookupResult>,
    _: Location,
    args: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [types] = matched.as_slice() {
      if let LookupResultWrapper::OptionalType(types) = &types.wrapper {
        return Ok(Self::from_matched(types));
      }
    }
    Err(ParserError::internal("Type", args))
  }

  fn from_matched(types: &OptionalTypeResult) -> Box<Node> {
    match types {
      // OptionalTypeResult::None => todo!(),
      OptionalTypeResult::Single(token) => {
        let start_loc = token.get_location().unwrap();
        let len = token.value.len();
        let end_loc = Location {
          line: start_loc.line,
          column: start_loc.column + len,
          offset: start_loc.offset + len,
        };
        TypeNode::new(
          false,
          token.value.to_owned(),
          Some(RangeLocation {
            start: start_loc,
            end: end_loc,
          })
        )
      }
      OptionalTypeResult::Nullable(nullable, token) => {
        let start_loc = nullable.get_location().unwrap();
        let end_loc = token.get_location().unwrap();
        let len = token.value.len();
        let end_loc = Location {
          line: end_loc.line,
          column: end_loc.column + len,
          offset: end_loc.offset + len,
        };
        TypeNode::new(
          true,
          token.value.to_owned(),
          Some(RangeLocation {
            start: start_loc,
            end: end_loc,
          })
        )
      }
      OptionalTypeResult::Union(vec) => {
        let mut start_loc = None;
        let mut end_loc = None;
        let items_last_index = vec.len() - 1;
        let items = vec
          .iter()
          .enumerate()
          .map(|(i, x)| {
            let parsed = Self::from_matched(x);
            if let Some(loc) = &parsed.loc {
              if i == items_last_index {
                end_loc = Some(loc.end.clone());
              }
              if i == 0 {
                start_loc = Some(loc.end.clone());
              }
            }
            parsed
          })
          .collect::<Vec<Box<Node>>>();
        UnionTypeNode::new(
          items,
          Some(RangeLocation { start: start_loc.unwrap(), end: end_loc.unwrap() })
        )
      }
      OptionalTypeResult::Intersection(vec) => {
        let mut start_loc = None;
        let mut end_loc = None;
        let items_last_index = vec.len() - 1;
        let items = vec
          .iter()
          .enumerate()
          .map(|(i, x)| {
            let parsed = Self::from_matched(x);
            if let Some(loc) = &parsed.loc {
              if i == items_last_index {
                end_loc = Some(loc.end.clone());
              }
              if i == 0 {
                start_loc = Some(loc.end.clone());
              }
            }
            parsed
          })
          .collect::<Vec<Box<Node>>>();
        IntersectionTypeNode::new(
          items,
          Some(RangeLocation { start: start_loc.unwrap(), end: end_loc.unwrap() })
        )
      }
      _ => {
        panic!("TypeParser::from_matched: failed to get type");
      }
    }
  }
}
