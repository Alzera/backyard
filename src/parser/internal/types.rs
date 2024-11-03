use crate::{
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

impl TypesParser {
  pub fn new(is_nullable: &Vec<Token>, type_name: &Vec<Token>) -> Option<Node> {
    if let Some(type_name) = type_name.get(0) {
      if [TokenType::Type, TokenType::Identifier].contains(&type_name.token_type) {
        let is_nullable =
          is_nullable.len() > 0 &&
          is_nullable.get(0).unwrap().token_type == TokenType::QuestionMark;
        return Some(TypeNode::new(is_nullable, type_name.value.to_owned()));
      }
    }
    None
  }
}
impl Internal for TypesParser {
  fn test(&self, tokens: &Vec<Token>, _: &LoopArgument) -> Option<Vec<Vec<Token>>> {
    match_pattern(
      tokens,
      [
        Lookup::Optional(vec![TokenType::QuestionMark]),
        Lookup::Equal(vec![TokenType::Type, TokenType::Identifier]),
      ].to_vec()
    )
  }

  fn parse(&self, _: &mut Parser, matched: Vec<Vec<Token>>, _: &LoopArgument) -> Option<Node> {
    if let [is_nullable, type_name] = matched.as_slice() {
      return TypesParser::new(is_nullable, type_name);
    }
    None
  }
}
