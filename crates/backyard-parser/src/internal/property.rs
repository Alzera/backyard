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
    let modifiers_rule = [
      [TokenType::Public, TokenType::Private, TokenType::Protected].to_vec(),
      [TokenType::Static, TokenType::Readonly].to_vec(),
    ];
    let mut modifiers = vec![vec![], vec![]];
    let mut pos = 0;
    loop {
      let token = tokens.get(pos);
      pos += 1;
      if pos > 2 || token.is_none() {
        break;
      }
      let token = token.unwrap();
      for (i, modifier) in modifiers_rule.iter().enumerate() {
        if modifiers[i].len() > 0 {
          continue;
        }
        if modifier.contains(&token.token_type) {
          modifiers[i].push(token.clone());
          break;
        }
      }
    }

    // need to manually check for variable type
    let first_test_count = modifiers
      .iter()
      .map(|i| i.len())
      .sum();
    let tmp_tokens = guard!(tokens.get(first_test_count..), {
      return None;
    }).to_vec();
    let type_test = TypesParser::test(&tmp_tokens, args);
    let type_test_count: usize = if let Some(t) = type_test {
      t.iter()
        .map(|i| i.len())
        .sum()
    } else {
      0
    };
    let tmp_tokens = guard!(tmp_tokens.get(type_test_count..), {
      return None;
    }).to_vec();
    guard!(match_pattern(&tmp_tokens, [Lookup::Equal(vec![TokenType::Variable])].to_vec()), {
      return None;
    });
    return Some(modifiers);
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
