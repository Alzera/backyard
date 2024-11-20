use backyard_lexer::token::{ Token, TokenType };
use backyard_nodes::node::{ Node, TypeNode };
use utils::guard;

use crate::{
  error::ParserError,
  parser::{ LoopArgument, Parser },
  utils::{ match_pattern, Lookup },
};

#[derive(Debug, Clone)]
pub struct TypesParser {}

impl TypesParser {
  #[allow(unused_assignments)]
  pub fn test(tokens: &Vec<Token>, _: &mut LoopArgument) -> Option<Vec<Vec<Token>>> {
    if
      let Some(m) = match_pattern(
        tokens,
        [
          Lookup::Equal(vec![TokenType::QuestionMark]),
          Lookup::Equal(vec![TokenType::Type, TokenType::Identifier]),
        ].to_vec()
      )
    {
      return Some(m);
    }
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
          [
            TokenType::Identifier,
            TokenType::Type,
            TokenType::Callable,
            TokenType::True,
            TokenType::False,
          ].contains(&token.token_type)) ||
        (last_token_type.is_some() &&
          [
            TokenType::Identifier,
            TokenType::Type,
            TokenType::Callable,
            TokenType::True,
            TokenType::False,
          ].contains(&last_token_type.unwrap()) &&
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
    Some(vec![matched])
  }

  pub fn parse(
    _: &mut Parser,
    matched: Vec<Vec<Token>>,
    args: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if matched.len() == 2 {
      if let [is_nullable, type_name] = matched.as_slice() {
        if let Some(type_name) = type_name.get(0) {
          if [TokenType::Type, TokenType::Identifier].contains(&type_name.token_type) {
            let is_nullable =
              is_nullable.len() > 0 &&
              is_nullable.get(0).unwrap().token_type == TokenType::QuestionMark;
            return Ok(TypeNode::new(is_nullable, vec![type_name.value.to_owned()]));
          }
        }
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
