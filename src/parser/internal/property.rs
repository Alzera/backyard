use std::vec;

use crate::{
  guard,
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
  fn test(&self, tokens: &Vec<Token>, args: &LoopArgument) -> Option<Vec<Vec<Token>>> {
    if
      let Some(first_test) = match_pattern(
        tokens,
        [
          Lookup::Optional(vec![TokenType::Public, TokenType::Private, TokenType::Protected]),
          Lookup::Optional(vec![TokenType::Static]),
        ].to_vec()
      )
    {
      let first_test_count = first_test
        .iter()
        .map(|i| i.len())
        .sum();
      let tmp_tokens = guard!(tokens.get(first_test_count..)).to_vec();
      let type_test = (TypesParser {}).test(&tmp_tokens, args);
      let type_test_count: usize = if type_test.is_none() {
        0
      } else {
        type_test
          .iter()
          .map(|i| i.len())
          .sum()
      };
      let tmp_tokens_index = type_test_count + first_test_count;
      let tmp_tokens = guard!(tokens.get(tmp_tokens_index..)).to_vec();
      guard!(match_pattern(&tmp_tokens, [Lookup::Equal(vec![TokenType::Variable])].to_vec()));
      return Some(first_test);
    }
    None
  }

  fn parse(&self, parser: &mut Parser, matched: Vec<Vec<Token>>, _: &LoopArgument) -> Option<Node> {
    if let [visibility, is_static] = matched.as_slice() {
      let items = parser.get_children(
        &mut LoopArgument::new(
          "property",
          &[TokenType::Comma],
          &[TokenType::Semicolon],
          &[
            ParserInternal::Comment(CommentParser {}),
            ParserInternal::Type(TypesParser {}),
            ParserInternal::PropertyItem(PropertyItemParser {}),
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
        Lookup::Equal(vec![TokenType::Variable]),
        Lookup::Optional(vec![TokenType::Assignment]),
      ].to_vec()
    )
  }

  fn parse(
    &self,
    parser: &mut Parser,
    matched: Vec<Vec<Token>>,
    args: &LoopArgument
  ) -> Option<Node> {
    if let [name, has_value] = matched.as_slice() {
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
        PropertyItemNode::new(
          IdentifierParser::from_matched(name),
          args.last_expr.to_owned(),
          value
        )
      );
    }
    None
  }
}
