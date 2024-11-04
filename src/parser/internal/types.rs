use crate::{
  guard,
  lexer::token::{ Token, TokenType },
  parser::{
    node::Node,
    nodes::types::TypeNode,
    parser::{ Internal, LoopArgument, Parser },
    utils::{ match_pattern, Lookup },
  },
};

#[derive(Debug, Clone)]
pub struct TypesParser {}

// impl TypesParser {
//   pub fn new(is_nullable: &Vec<Token>, type_name: &Vec<Token>) -> Option<Node> {
//     if let Some(type_name) = type_name.get(0) {
//       if [TokenType::Type, TokenType::Identifier].contains(&type_name.token_type) {
//         let is_nullable =
//           is_nullable.len() > 0 &&
//           is_nullable.get(0).unwrap().token_type == TokenType::QuestionMark;
//         return Some(TypeNode::new(is_nullable, type_name.value.to_owned()));
//       }
//     }
//     None
//   }
// }
impl Internal for TypesParser {
  #[allow(unused_assignments)]
  fn test(&self, tokens: &Vec<Token>, _: &LoopArgument) -> Option<Vec<Vec<Token>>> {
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
      let token = guard!(tokens.get(index));
      index += 1;
      if
        ((last_token_type == None || last_token_type.unwrap() == TokenType::BitwiseOr) &&
          [TokenType::Identifier, TokenType::Type].contains(&token.token_type)) ||
        (last_token_type.is_some() &&
          [TokenType::Identifier, TokenType::Type].contains(&last_token_type.unwrap()) &&
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

  fn parse(&self, _: &mut Parser, matched: Vec<Vec<Token>>, _: &LoopArgument) -> Option<Node> {
    if matched.len() == 2 {
      if let [is_nullable, type_name] = matched.as_slice() {
        if let Some(type_name) = type_name.get(0) {
          if [TokenType::Type, TokenType::Identifier].contains(&type_name.token_type) {
            let is_nullable =
              is_nullable.len() > 0 &&
              is_nullable.get(0).unwrap().token_type == TokenType::QuestionMark;
            return Some(TypeNode::new(is_nullable, vec![type_name.value.to_owned()]));
          }
        }
      }
    } else if matched.len() == 1 {
      if let [types] = matched.as_slice() {
        return Some(
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
    None
  }
}
