use crate::{
  lexer::token::{ Token, TokenType },
  parser::{
    node::Node,
    nodes::property::{ PropertyItemNode, PropertyNode },
    parser::{ Internal, LoopArgument, Parser, ParserInternal },
    utils::{ match_pattern, some_or_default, Lookup },
  },
};

use super::{ comment::CommentParser, identifier::IdentifierParser, types::TypesParser };

#[derive(Debug, Clone)]
pub struct PropertyParser {}

impl Internal for PropertyParser {
  fn test(&self, tokens: &Vec<Token>, _: &LoopArgument) -> Option<Vec<Vec<Token>>> {
    match_pattern(
      tokens,
      [
        Lookup::Optional(vec![TokenType::Public, TokenType::Private, TokenType::Protected]),
        Lookup::Optional(vec![TokenType::Static]),
        Lookup::Optional(vec![TokenType::QuestionMark]),
        Lookup::Optional(vec![TokenType::Type, TokenType::Identifier]),
        Lookup::Equal(vec![TokenType::Variable]),
      ].to_vec()
    )
  }

  fn parse(&self, parser: &mut Parser, matched: Vec<Vec<Token>>, _: &LoopArgument) -> Option<Node> {
    if let [visibility, is_static, is_nullable, type_name, _] = matched.as_slice() {
      parser.position -= 1;
      if is_nullable.len() > 0 {
        parser.position -= 1;
      }
      if type_name.len() > 0 {
        parser.position -= 1;
      }
      let items = parser.get_children(
        &mut LoopArgument::new(
          "property",
          &[TokenType::Comma],
          &[TokenType::Semicolon],
          &[
            ParserInternal::PropertyItem(PropertyItemParser {}),
            ParserInternal::Comment(CommentParser {}),
          ]
        )
      );
      return Some(
        PropertyNode::new(
          some_or_default(visibility.get(0), String::from(""), |i| i.value.to_owned()),
          is_static.len() > 0,
          items
        )
      );
    }
    None
  }
}

#[derive(Debug, Clone)]
pub struct PropertyItemParser {}

impl Internal for PropertyItemParser {
  fn test(&self, tokens: &Vec<Token>, _: &LoopArgument) -> Option<Vec<Vec<Token>>> {
    match_pattern(
      tokens,
      [
        Lookup::Optional(vec![TokenType::QuestionMark]),
        Lookup::Optional(vec![TokenType::Type, TokenType::Identifier]),
        Lookup::Equal(vec![TokenType::Variable]),
        Lookup::Optional(vec![TokenType::Assignment]),
      ].to_vec()
    )
  }

  fn parse(&self, parser: &mut Parser, matched: Vec<Vec<Token>>, _: &LoopArgument) -> Option<Node> {
    if let [is_nullable, type_name, name, has_value] = matched.as_slice() {
      let variable_type = TypesParser::new(is_nullable, type_name);
      let value = if has_value.len() > 0 {
        parser.get_statement(
          &mut LoopArgument::with_tokens(
            "property_item",
            &[TokenType::Comma, TokenType::Semicolon],
            &[]
          )
        )
      } else {
        None
      };
      return Some(
        PropertyItemNode::new(IdentifierParser::from_matched(name), variable_type, value)
      );
    }
    None
  }
}
