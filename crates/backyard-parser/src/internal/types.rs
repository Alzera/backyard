use backyard_lexer::token::{ Token, TokenType };
use backyard_nodes::node::{ IntersectionTypeNode, Node, TypeNode, UnionTypeNode };
use utils::guard;

use crate::{ error::ParserError, parser::{ LoopArgument, Parser } };

#[derive(Debug, Clone)]
pub struct TypesParser;

impl TypesParser {
  #[allow(unused_assignments)]
  pub fn test(tokens: &[Token], _: &mut LoopArgument) -> Option<Vec<Vec<Token>>> {
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
    let mut pos = 0;
    let mut token = guard!(tokens.get(pos), {
      return None;
    });
    println!("matched 0: {:?}", token);
    let mut is_nullable = None;
    if token.token_type == TokenType::QuestionMark {
      is_nullable = Some(token.to_owned());
      pos += 1;
      token = guard!(tokens.get(pos), {
        return None;
      });
    }
    let is_nullable = if let Some(t) = is_nullable { vec![t.to_owned()] } else { vec![] };
    let next_token = guard!(tokens.get(pos + 1), {
      return None;
    });
    if
      is_nullable.is_empty() &&
      [TokenType::BitwiseOr, TokenType::BitwiseAnd].contains(&next_token.token_type)
    {
      let separator = next_token.token_type;
      let mut matched = vec![];
      let mut index = 0;
      let mut last_token_type = None;
      loop {
        let token = guard!(tokens.get(index), {
          return None;
        });
        index += 1;
        if
          ((last_token_type.is_none() || last_token_type.unwrap() == separator) &&
            TYPES.contains(&token.token_type)) ||
          (last_token_type.is_some() &&
            TYPES.contains(&last_token_type.unwrap()) &&
            token.token_type == separator)
        {
          last_token_type = Some(token.token_type);
          matched.push(token.to_owned());
          continue;
        }
        break;
      }
      println!("matched 1: {:?}", matched);
      if let Some(m) = matched.last() {
        if m.token_type == separator {
          matched.pop();
        }
      }
      println!("matched 2: {:?}", matched);
      if matched.is_empty() {
        return None;
      }
      return Some(vec![matched]);
    }
    if TYPES.contains(&token.token_type) {
      return Some(vec![is_nullable, vec![token.to_owned()]]);
    }
    None
  }

  pub fn parse(
    _: &mut Parser,
    matched: Vec<Vec<Token>>,
    args: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if matched.len() == 2 {
      if let [is_nullable, types] = matched.as_slice() {
        let name = guard!(types.first(), {
          return Err(ParserError::internal("Type", args));
        }).value.to_owned();
        return Ok(TypeNode::new(!is_nullable.is_empty(), name));
      }
    } else if matched.len() == 1 {
      if let [types] = matched.as_slice() {
        let mut separator = None;
        let types = types
          .iter()
          .filter_map(|i| {
            if [TokenType::BitwiseOr, TokenType::BitwiseAnd].contains(&i.token_type) {
              separator = Some(i.token_type);
              return None;
            }
            Some(i.value.to_owned())
          })
          .collect();
        if let Some(separator) = separator {
          if separator == TokenType::BitwiseOr {
            return Ok(UnionTypeNode::new(types));
          } else if separator == TokenType::BitwiseAnd {
            return Ok(IntersectionTypeNode::new(types));
          }
        } else if types.len() == 1 {
          return Ok(TypeNode::new(false, types.last().unwrap().to_owned()));
        }
      }
    }
    Err(ParserError::internal("Type", args))
  }
}
