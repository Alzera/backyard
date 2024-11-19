use backyard_lexer::token::{ Token, TokenType, TokenTypeArrayCombine };
use backyard_nodes::node::{ Node, PropertyItemNode, PropertyNode };
use utils::guard;

use crate::{
  error::ParserError,
  parser::{ LoopArgument, Parser },
  utils::{ match_pattern, some_or_default, Lookup },
};

use super::{ comment::CommentParser, identifier::IdentifierParser, types::TypesParser };

#[derive(Debug, Clone)]
pub struct PropertyParser {}

impl PropertyParser {
  pub fn test(tokens: &Vec<Token>, args: &mut LoopArgument) -> Option<Vec<Vec<Token>>> {
    if
      let Some(first_test) = match_pattern(
        tokens,
        [
          Lookup::Optional(vec![TokenType::Public, TokenType::Private, TokenType::Protected]),
          Lookup::Optional(vec![TokenType::Static, TokenType::Readonly]),
        ].to_vec()
      )
    {
      let first_test_count = first_test
        .iter()
        .map(|i| i.len())
        .sum();
      let tmp_tokens = guard!(tokens.get(first_test_count..), {
        return None;
      }).to_vec();
      let type_test = TypesParser::test(&tmp_tokens, args);
      let type_test_count: usize = if type_test.is_none() {
        0
      } else {
        type_test
          .iter()
          .map(|i| i.len())
          .sum()
      };
      let tmp_tokens_index = type_test_count + first_test_count;
      let tmp_tokens = guard!(tokens.get(tmp_tokens_index..), {
        return None;
      }).to_vec();
      guard!(match_pattern(&tmp_tokens, [Lookup::Equal(vec![TokenType::Variable])].to_vec()), {
        return None;
      });
      return Some(first_test);
    }
    None
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<Vec<Token>>,
    args: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [visibility, modifier] = matched.as_slice() {
      let items = parser.get_children(
        &mut LoopArgument::new(
          "property",
          &[TokenType::Comma],
          &[TokenType::Semicolon],
          &[
            (CommentParser::test, CommentParser::parse),
            (TypesParser::test, TypesParser::parse),
            (PropertyItemParser::test, PropertyItemParser::parse),
          ]
        )
      )?;
      return Ok(
        PropertyNode::new(
          some_or_default(visibility.get(0), String::from(""), |i| i.value.to_owned()),
          some_or_default(modifier.get(0), String::from(""), |i| i.value.to_owned()),
          items
        )
      );
    }
    Err(ParserError::internal("Property", args))
  }
}

#[derive(Debug, Clone)]
pub struct PropertyItemParser {}

impl PropertyItemParser {
  pub fn test(tokens: &Vec<Token>, _: &mut LoopArgument) -> Option<Vec<Vec<Token>>> {
    match_pattern(
      tokens,
      [
        Lookup::Equal(vec![TokenType::Variable]),
        Lookup::Optional(vec![TokenType::Assignment]),
      ].to_vec()
    )
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<Vec<Token>>,
    args: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [name, has_value] = matched.as_slice() {
      let value = if has_value.len() > 0 {
        parser.get_statement(
          &mut LoopArgument::with_tokens(
            "property_item",
            &args.separators.combine(&[TokenType::Comma, TokenType::Semicolon]),
            args.breakers
          )
        )?
      } else {
        None
      };
      return Ok(
        PropertyItemNode::new(
          IdentifierParser::from_matched(name),
          args.last_expr.to_owned(),
          value
        )
      );
    }
    Err(ParserError::internal("PropertyItem", args))
  }
}
