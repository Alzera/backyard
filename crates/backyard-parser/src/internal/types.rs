use backyard_lexer::token::{ Token, TokenType, TokenTypeArrayCombine };
use backyard_nodes::node::{ Node, TypeNode };
use utils::guard;

use crate::{ error::ParserError, parser::{ LoopArgument, Parser } };

#[derive(Debug, Clone)]
pub struct TypesParser {}

impl TypesParser {
  #[allow(unused_assignments)]
  pub fn test(tokens: &Vec<Token>, _: &mut LoopArgument) -> Option<Vec<Vec<Token>>> {
    const TYPES: [TokenType; 9] = [
      TokenType::Identifier,
      TokenType::Type,
      TokenType::Callable,
      TokenType::Static,
      TokenType::SelfKeyword,
      TokenType::Array,
      TokenType::True,
      TokenType::False,
      TokenType::Null,
    ];
    let mut pos = 0;
    let mut token = guard!(tokens.get(pos), {
      return None;
    });
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
    if [TokenType::Identifier, TokenType::Name].contains(&token.token_type) {
      if next_token.token_type == TokenType::Name {
        let mut name = vec![token.to_owned(), next_token.to_owned()];
        pos += 2;
        loop {
          if let Some(token) = tokens.get(pos) {
            if [TokenType::Name].contains(&token.token_type) {
              name.push(token.to_owned());
              pos += 1;
              continue;
            }
          }
          break;
        }
        return Some(vec![is_nullable, name]);
      }
    }
    if next_token.token_type == TokenType::BitwiseOr {
      let mut matched = vec![];
      let mut index = 0;
      let mut last_token_type = None;
      loop {
        let token = guard!(tokens.get(index), {
          return None;
        });
        index += 1;
        if
          ((last_token_type == None || last_token_type.unwrap() == TokenType::BitwiseOr) &&
            TYPES.contains(&token.token_type)) ||
          (last_token_type.is_some() &&
            TYPES.contains(&last_token_type.unwrap()) &&
            token.token_type == TokenType::BitwiseOr)
        {
          last_token_type = Some(token.token_type);
          matched.push(token.to_owned());
          continue;
        }
        break;
      }
      if matched.len() == 0 {
        return None;
      }
      return Some(vec![matched]);
    }
    if TYPES.combine(&[TokenType::Name]).contains(&token.token_type) {
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
      if let [is_nullable, names] = matched.as_slice() {
        let name = names
          .iter()
          .map(|x| x.value.to_owned())
          .collect();
        return Ok(TypeNode::new(is_nullable.len() > 0, vec![name]));
      }
    } else if matched.len() == 1 {
      if let [types] = matched.as_slice() {
        return Ok(
          TypeNode::new(
            false,
            types
              .iter()
              .filter_map(|i| {
                if i.token_type == TokenType::BitwiseOr {
                  return None;
                }
                Some(i.value.to_owned())
              })
              .collect()
          )
        );
      }
    }
    Err(ParserError::internal("Type", args))
  }
}
