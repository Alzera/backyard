use bumpalo::vec;
use backyard_lexer::token::TokenType;
use backyard_nodes::{
  Location,
  Modifier,
  Node,
  PropertyHookNode,
  PropertyItemNode,
  PropertyNode,
  Visibility,
  utils::{ IntoBoxedNode, IntoBoxedOptionNode },
};

use crate::{
  error::ParserError,
  parser::{ LocationHelper, LoopArgument, OptionNodeOrInternal, Parser, TokenTypeArrayCombine },
  utils::{ match_pattern, Lookup, LookupResult, ModifierLookup },
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
  pub fn test<'arena, 'a>(
    parser: &mut Parser<'arena, 'a>,
    _: &mut LoopArgument
  ) -> Option<std::vec::Vec<LookupResult<'arena>>> {
    match_pattern(
      parser,
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

  pub fn parse<'arena, 'a, 'b>(
    parser: &mut Parser<'arena, 'a>,
    mut matched: std::vec::Vec<LookupResult<'arena>>,
    start_loc: Location,
    _: &mut LoopArgument<'arena, 'b>
  ) -> Result<Node<'arena>, ParserError> {
    if let [modifiers, has_var, prop_type, name] = matched.as_mut_slice() {
      let next_token = parser.get_token(parser.position)?;
      let prop_type = prop_type.as_optional_type().into_boxed(parser.arena);
      let first_prop = if next_token.token_type == TokenType::Assignment {
        parser.position += 1;
        if
          let Some(value) = parser.get_statement(
            &mut LoopArgument::with_tokens(
              parser.arena,
              "property",
              &[TokenType::Comma, TokenType::Semicolon, TokenType::LeftCurlyBracket],
              &[]
            )
          )?
        {
          let name = name.as_equal(parser)?;
          let item_start_loc = name.get_location().unwrap();
          PropertyItemNode::loc(
            IdentifierParser::from_token(name).into_boxed(parser.arena),
            prop_type,
            Some(value.into_boxed(parser.arena)),
            parser.gen_loc(item_start_loc)
          )
        } else {
          return Err(ParserError::Internal);
        }
      } else {
        let name = name.as_equal(parser)?;
        let item_start_loc = name.get_location().unwrap();
        PropertyItemNode::loc(
          IdentifierParser::from_token(name).into_boxed(parser.arena),
          prop_type,
          None,
          parser.gen_loc(item_start_loc)
        )
      };
      let mut items = vec![in parser.arena; first_prop];
      let mut hooks = vec![in parser.arena];
      let next_token = parser.get_token(parser.position)?;
      if next_token.token_type == TokenType::Comma {
        let next_items = parser.get_children(
          &mut LoopArgument::new(
            parser.arena,
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
            parser.arena,
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
      let mut visibilities = std::vec![];
      let mut modifier = None;
      if let Some([m0, m1]) = modifiers.as_modifier() {
        visibilities = m0.as_visibilities(parser);
        modifier = m1.as_custom(parser, |x| Modifier::try_from(x));
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
  pub fn test<'arena, 'a>(
    parser: &mut Parser<'arena, 'a>,
    _: &mut LoopArgument
  ) -> Option<std::vec::Vec<LookupResult<'arena>>> {
    match_pattern(
      parser,
      &[Lookup::Equal(&[TokenType::Variable]), Lookup::Optional(&[TokenType::Assignment])]
    )
  }

  pub fn parse<'arena, 'a, 'b>(
    parser: &mut Parser<'arena, 'a>,
    matched: std::vec::Vec<LookupResult<'arena>>,
    start_loc: Location,
    args: &mut LoopArgument<'arena, 'b>
  ) -> Result<Node<'arena>, ParserError> {
    if let [name, has_value] = matched.as_slice() {
      let name = IdentifierParser::from_token(name.as_equal(parser)?);
      let value = if !has_value.is_empty() {
        parser.get_statement(
          &mut LoopArgument::with_tokens(
            parser.arena,
            "property_item",
            &args.separators.combine(&[TokenType::Comma, TokenType::Semicolon]),
            args.breakers
          )
        )?
      } else {
        None
      };
      return Ok(
        PropertyItemNode::loc(
          name.into_boxed(parser.arena),
          args.last_expr.take().into_boxed(parser.arena),
          value.into_boxed(parser.arena),
          parser.gen_loc(start_loc)
        )
      );
    }
    Err(ParserError::Internal)
  }
}

#[derive(Debug, Clone)]
pub struct HookParser;

impl HookParser {
  pub fn test<'arena, 'a>(
    parser: &mut Parser<'arena, 'a>,
    _: &mut LoopArgument
  ) -> Option<std::vec::Vec<LookupResult<'arena>>> {
    match_pattern(
      parser,
      &[
        Lookup::Optional(&[TokenType::BitwiseAnd]),
        Lookup::Equal(&[TokenType::Get, TokenType::Set]),
        Lookup::Optional(&[TokenType::LeftParenthesis]),
      ]
    )
  }

  pub fn parse<'arena, 'a, 'b>(
    parser: &mut Parser<'arena, 'a>,
    matched: std::vec::Vec<LookupResult<'arena>>,
    start_loc: Location,
    _: &mut LoopArgument<'arena, 'b>
  ) -> Result<Node<'arena>, ParserError> {
    if let [is_ref, name, has_param] = matched.as_slice() {
      let is_get = name.as_equal(parser)?.token_type == TokenType::Get;
      let params = if !is_get && !has_param.is_empty() {
        FunctionParser::get_parameters(parser)?
      } else {
        vec![in parser.arena]
      };
      let next_token = parser.get_token(parser.position)?;
      let body = if next_token.token_type == TokenType::LeftCurlyBracket {
        BlockParser::new_block(parser)?
      } else if next_token.token_type == TokenType::Arrow {
        parser.position += 1;
        let expr = parser
          .get_statement(
            &mut LoopArgument::with_tokens(parser.arena, "set_hook", &[], &[TokenType::Semicolon])
          )?
          .ok_internal()?;
        parser.position += 1;
        expr
      } else {
        return Err(ParserError::Internal);
      };
      return Ok(
        PropertyHookNode::loc(
          is_get,
          !is_ref.is_empty(),
          params,
          body.into_boxed(parser.arena),
          parser.gen_loc(start_loc)
        )
      );
    }
    Err(ParserError::Internal)
  }
}
