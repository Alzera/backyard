use backyard_lexer::token::{ Token, TokenType };
use backyard_nodes::node::{
  Location,
  Modifier,
  Node,
  PropertyHookNode,
  PropertyItemNode,
  PropertyNode,
  Visibility,
};

use crate::{
  cast_lookup_result,
  error::ParserError,
  guard,
  parser::{ LocationHelper, LoopArgument, Parser, TokenTypeArrayCombine },
  utils::{
    match_pattern,
    Lookup,
    LookupResult,
    LookupResultWrapper,
    ModifierLookup,
    ModifierResult,
  },
};

use super::{
  block::BlockParser,
  comment::CommentParser,
  function::FunctionParser,
  identifier::IdentifierParser,
  types::TypesParser,
};

#[derive(Debug, Clone)]
pub struct PropertyParser;

impl PropertyParser {
  pub fn test(tokens: &[Token], _: &mut LoopArgument) -> Option<Vec<LookupResult>> {
    match_pattern(
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
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<LookupResult>,
    start_loc: Location,
    _: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [modifiers, has_var, prop_type, name] = matched.as_slice() {
      let next_token = guard!(parser.tokens.get(parser.position));
      let name = cast_lookup_result!(Equal, &name.wrapper);
      let prop_type = cast_lookup_result!(OptionalType, &prop_type.wrapper);
      let first_prop = if next_token.token_type == TokenType::Assignment {
        parser.position += 1;
        if
          let Some(value) = parser.get_statement(
            &mut LoopArgument::with_tokens(
              "property",
              &[TokenType::Comma, TokenType::Semicolon, TokenType::LeftCurlyBracket],
              &[]
            )
          )?
        {
          let item_start_loc = name.get_location().unwrap();
          PropertyItemNode::loc(
            IdentifierParser::from_token(name),
            prop_type.to_owned(),
            Some(value),
            parser.gen_loc(item_start_loc)
          )
        } else {
          return Err(ParserError::Internal);
        }
      } else {
        let item_start_loc = name.get_location().unwrap();
        PropertyItemNode::loc(
          IdentifierParser::from_token(name),
          prop_type.to_owned(),
          None,
          parser.gen_loc(item_start_loc)
        )
      };
      let mut items = vec![first_prop];
      let mut hooks = vec![];
      let next_token = guard!(parser.tokens.get(parser.position));
      if next_token.token_type == TokenType::Comma {
        let next_items = parser.get_children(
          &mut LoopArgument::new(
            "property",
            &[TokenType::Comma],
            &[TokenType::Semicolon, TokenType::LeftCurlyBracket],
            &[
              (CommentParser::test, CommentParser::parse),
              (TypesParser::test, TypesParser::parse),
              (PropertyItemParser::test, PropertyItemParser::parse),
            ]
          )
        )?;
        items.extend(next_items);
      } else if next_token.token_type == TokenType::LeftCurlyBracket {
        parser.position += 1;
        hooks = parser.get_children(
          &mut LoopArgument::new(
            "property",
            &[],
            &[TokenType::RightCurlyBracket],
            &[
              (CommentParser::test, CommentParser::parse),
              (HookParser::test, HookParser::parse),
            ]
          )
        )?;
      }
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
      return Ok(PropertyNode::loc(visibilities, modifier, hooks, items, parser.gen_loc(start_loc)));
    }
    Err(ParserError::Internal)
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
        return Err(ParserError::Internal);
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
        PropertyItemNode::loc(name, args.last_expr.to_owned(), value, parser.gen_loc(start_loc))
      );
    }
    Err(ParserError::Internal)
  }
}

#[derive(Debug, Clone)]
pub struct HookParser;

impl HookParser {
  pub fn test(tokens: &[Token], _: &mut LoopArgument) -> Option<Vec<LookupResult>> {
    match_pattern(
      tokens,
      &[
        Lookup::Optional(&[TokenType::BitwiseAnd]),
        Lookup::Equal(&[TokenType::Get, TokenType::Set]),
        Lookup::Optional(&[TokenType::LeftParenthesis]),
      ]
    )
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<LookupResult>,
    start_loc: Location,
    _: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [is_ref, name, has_param] = matched.as_slice() {
      let is_get = if let LookupResultWrapper::Equal(name) = &name.wrapper {
        name.token_type == TokenType::Get
      } else {
        return Err(ParserError::Internal);
      };
      let mut params = vec![];
      if !is_get {
        if let LookupResultWrapper::Optional(Some(_)) = &has_param.wrapper {
          params = FunctionParser::get_parameters(parser)?;
        }
      }
      if let Some(next_token) = parser.tokens.get(parser.position) {
        let body = if next_token.token_type == TokenType::LeftCurlyBracket {
          BlockParser::new_block(parser)?
        } else if next_token.token_type == TokenType::Arrow {
          parser.position += 1;
          if
            let Some(expr) = parser.get_statement(
              &mut LoopArgument::with_tokens("set_hook", &[], &[TokenType::Semicolon])
            )?
          {
            parser.position += 1;
            expr
          } else {
            return Err(ParserError::Internal);
          }
        } else {
          return Err(ParserError::Internal);
        };
        return Ok(
          PropertyHookNode::loc(is_get, !is_ref.is_empty(), params, body, parser.gen_loc(start_loc))
        );
      }
    }
    Err(ParserError::Internal)
  }
}
