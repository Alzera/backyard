use backyard_lexer::token::{ Token, TokenType, TokenTypeArrayCombine };
use backyard_nodes::node::{ Location, Modifier, Node, PropertyItemNode, PropertyNode, Visibility };

use crate::{
  error::ParserError,
  guard,
  parser::{ LoopArgument, Parser },
  utils::{ match_pattern, Lookup },
};

use super::{ comment::CommentParser, identifier::IdentifierParser, types::TypesParser };

#[derive(Debug, Clone)]
pub struct PropertyParser;

impl PropertyParser {
  pub fn test(tokens: &[Token], args: &mut LoopArgument) -> Option<Vec<Vec<Token>>> {
    let modifiers_rule = [
      [TokenType::Public, TokenType::Private, TokenType::Protected].to_vec(),
      [TokenType::Static, TokenType::Readonly].to_vec(),
    ];
    let mut modifiers = vec![vec![], vec![]];
    let mut pos = 0;
    loop {
      let token = tokens.get(pos);
      if token.is_none() {
        return None;
      }
      pos += 1;
      if pos > 2 {
        break;
      }
      let mut assigned = false;
      let token = token.unwrap();
      for (i, modifier) in modifiers_rule.iter().enumerate() {
        if !modifiers[i].is_empty() {
          continue;
        }
        if modifier.contains(&token.token_type) {
          modifiers[i].push(token.clone());
          assigned = true;
          break;
        }
      }
      if !assigned {
        break;
      }
    }
    let first_test_count = modifiers
      .iter()
      .map(|i| i.len())
      .sum();
    if
      let Some(has_var) = match_pattern(
        &tokens[first_test_count..],
        &[Lookup::Equal(&[TokenType::Var])]
      )
    {
      if let Some(has_var) = has_var.first() {
        modifiers.push(has_var.to_owned());
      }
    } else {
      modifiers.push(vec![]);
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
    guard!(match_pattern(&tmp_tokens, &[Lookup::Equal(&[TokenType::Variable])]), {
      return None;
    });
    Some(modifiers)
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<Vec<Token>>,
    start_loc: Location,
    args: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [visibility, modifier, has_var] = matched.as_slice() {
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
      let mut visibility = Visibility::from_str(
        &visibility
          .first()
          .map(|i| i.value.to_owned())
          .unwrap_or_default()
      );
      if visibility.is_none() && !has_var.is_empty() {
        visibility = Some(Visibility::Public);
      }
      return Ok(
        PropertyNode::new(
          visibility,
          Modifier::from_str(
            &modifier
              .first()
              .map(|i| i.value.to_owned())
              .unwrap_or_default()
          ),
          items,
          parser.gen_loc(start_loc)
        )
      );
    }
    Err(ParserError::internal("Property", args))
  }
}

#[derive(Debug, Clone)]
pub struct PropertyItemParser;

impl PropertyItemParser {
  pub fn test(tokens: &[Token], _: &mut LoopArgument) -> Option<Vec<Vec<Token>>> {
    match_pattern(
      tokens,
      &[Lookup::Equal(&[TokenType::Variable]), Lookup::Optional(&[TokenType::Assignment])]
    )
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<Vec<Token>>,
    start_loc: Location,
    args: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [name, has_value] = matched.as_slice() {
      let value = if !has_value.is_empty() {
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
          value,
          parser.gen_loc(start_loc)
        )
      );
    }
    Err(ParserError::internal("PropertyItem", args))
  }
}
