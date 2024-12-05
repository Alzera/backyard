use backyard_lexer::token::{ Token, TokenType };
use backyard_nodes::node::{ Location, Modifier, Node, PropertyItemNode, PropertyNode, Visibility };

use crate::{
  error::ParserError,
  parser::{ LoopArgument, Parser, TokenTypeArrayCombine },
  utils::{
    match_pattern,
    Lookup,
    LookupResult,
    LookupResultWrapper,
    ModifierLookup,
    ModifierResult,
  },
};

use super::{ comment::CommentParser, identifier::IdentifierParser, types::TypesParser };

#[derive(Debug, Clone)]
pub struct PropertyParser;

impl PropertyParser {
  pub fn test(tokens: &[Token], _: &mut LoopArgument) -> Option<Vec<LookupResult>> {
    if
      let Some(m) = match_pattern(
        tokens,
        &[
          Lookup::Modifiers(
            &[
              ModifierLookup::Visibility,
              ModifierLookup::Custom(&[TokenType::Static, TokenType::Readonly]),
            ]
          ),
          Lookup::Optional(&[TokenType::Var]),
          Lookup::OptionalType,
          Lookup::Equal(&[TokenType::Variable]),
        ]
      )
    {
      return Some(m[..2].to_vec());
    }
    None
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<LookupResult>,
    start_loc: Location,
    args: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [modifiers, has_var] = matched.as_slice() {
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
      let mut visibilities = vec![];
      let mut modifier = None;
      if let LookupResultWrapper::Modifier(modifiers) = &modifiers.wrapper {
        if
          let [
            ModifierResult::Visibility(visibilities_modifier),
            ModifierResult::Custom(modifier_modifier),
          ] = modifiers.as_slice()
        {
          visibilities = visibilities_modifier
            .iter()
            .filter_map(|x| Visibility::try_parse(&x.value))
            .collect();
          modifier = Modifier::try_parse(
            &modifier_modifier
              .as_ref()
              .map(|i| i.value.to_owned())
              .unwrap_or_default()
          );
        }
      }
      if visibilities.is_empty() && !has_var.is_empty() {
        visibilities.push(Visibility::Public);
      }
      return Ok(PropertyNode::new(visibilities, modifier, items, parser.gen_loc(start_loc)));
    }
    Err(ParserError::internal("Property", args))
  }
}

#[derive(Debug, Clone)]
pub struct PropertyItemParser;

impl PropertyItemParser {
  pub fn test(tokens: &[Token], _: &mut LoopArgument) -> Option<Vec<LookupResult>> {
    match_pattern(
      tokens,
      &[Lookup::Equal(&[TokenType::Variable]), Lookup::Optional(&[TokenType::Assignment])]
    )
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<LookupResult>,
    start_loc: Location,
    args: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [name, has_value] = matched.as_slice() {
      let name = if let LookupResultWrapper::Equal(name) = &name.wrapper {
        IdentifierParser::from_token(name)
      } else {
        return Err(ParserError::internal("PropertyItem", args));
      };
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
        PropertyItemNode::new(name, args.last_expr.to_owned(), value, parser.gen_loc(start_loc))
      );
    }
    Err(ParserError::internal("PropertyItem", args))
  }
}
