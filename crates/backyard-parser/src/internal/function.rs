use backyard_lexer::token::{ Token, TokenType };
use backyard_nodes::node::{
  AnonymousFunctionNode,
  ArrowFunctionNode,
  ConstructorParameterNode,
  FunctionNode,
  Location,
  Modifier,
  Node,
  ParameterNode,
  Visibility,
};

use crate::{
  cast_lookup_result,
  error::ParserError,
  guard,
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

use super::{
  attribute::AttributeParser,
  block::BlockParser,
  comment::CommentParser,
  identifier::IdentifierParser,
  magic::MagicParser,
  types::TypesParser,
};

#[derive(Debug, Clone)]
pub struct FunctionParser;

impl FunctionParser {
  pub fn test(tokens: &[Token], _: &mut LoopArgument) -> Option<Vec<LookupResult>> {
    if
      let Some(m) = match_pattern(
        tokens,
        &[
          Lookup::Equal(&[TokenType::Function]),
          Lookup::Optional(&[TokenType::BitwiseAnd]),
          Lookup::Any,
          Lookup::Equal(&[TokenType::LeftParenthesis]),
        ]
      )
    {
      return Some(m);
    }
    if
      let Some(m) = match_pattern(
        tokens,
        &[
          Lookup::Equal(&[TokenType::Function]),
          Lookup::Optional(&[TokenType::BitwiseAnd]),
          Lookup::Equal(&[TokenType::LeftParenthesis]),
        ]
      )
    {
      return Some(m);
    }
    match_pattern(
      tokens,
      &[
        Lookup::Equal(&[TokenType::Fn]),
        Lookup::Optional(&[TokenType::BitwiseAnd]),
        Lookup::Equal(&[TokenType::LeftParenthesis]),
      ]
    )
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<LookupResult>,
    start_loc: Location,
    args: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    match matched.len() {
      4 => FunctionParser::parse_basic(parser, matched, start_loc),
      3 => {
        if let Some(f) = matched.first() {
          if let LookupResultWrapper::Equal(f) = &f.wrapper {
            if f.token_type == TokenType::Fn {
              return FunctionParser::parse_arrow(parser, matched, start_loc, args);
            } else if f.token_type == TokenType::Function {
              return FunctionParser::parse_anonymous(parser, matched, start_loc);
            }
          }
        }
        Err(ParserError::Internal)
      }
      _ => Err(ParserError::Internal),
    }
  }
}

impl FunctionParser {
  pub fn parse_arrow(
    parser: &mut Parser,
    matched: Vec<LookupResult>,
    start_loc: Location,
    args: &LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [_, is_ref, _] = matched.as_slice() {
      let arguments = FunctionParser::get_parameters(parser)?;
      let return_type = FunctionParser::get_return_type(parser).ok();
      parser.position += 1;
      let body = guard!(
        parser.get_statement(
          &mut LoopArgument::with_tokens(
            "function_arrow",
            &[],
            &args.breakers.combine(args.separators)
          )
        )?
      );
      return Ok(
        ArrowFunctionNode::loc(
          !is_ref.is_empty(),
          arguments,
          return_type,
          body,
          parser.gen_loc(start_loc)
        )
      );
    }
    Err(ParserError::Internal)
  }

  pub fn parse_anonymous(
    parser: &mut Parser,
    matched: Vec<LookupResult>,
    start_loc: Location
  ) -> Result<Box<Node>, ParserError> {
    if let [_, is_ref, _] = matched.as_slice() {
      let arguments = FunctionParser::get_parameters(parser)?;
      let mut uses = vec![];
      if let Some(next_token) = parser.tokens.get(parser.position) {
        if next_token.token_type == TokenType::Use {
          parser.position += 2;
          uses = parser.get_children(
            &mut LoopArgument::with_tokens(
              "function_anonymous",
              &[TokenType::Comma],
              &[TokenType::RightParenthesis]
            )
          )?;
        }
      }
      let return_type = FunctionParser::get_return_type(parser).ok();
      let body = BlockParser::new_block(parser)?;
      return Ok(
        AnonymousFunctionNode::loc(
          !is_ref.is_empty(),
          arguments,
          uses,
          return_type,
          body,
          parser.gen_loc(start_loc)
        )
      );
    }
    Err(ParserError::Internal)
  }

  pub fn parse_basic(
    parser: &mut Parser,
    matched: Vec<LookupResult>,
    start_loc: Location
  ) -> Result<Box<Node>, ParserError> {
    if let [_, is_ref, name, _] = matched.as_slice() {
      let mut is_contructor = false;
      let name = if let LookupResultWrapper::Any(name) = &name.wrapper {
        if name.token_type == TokenType::MagicMethod {
          if name.value == "__construct" {
            is_contructor = true;
          }
          MagicParser::from_token(name)
        } else {
          IdentifierParser::from_token(name)
        }
      } else {
        return Err(ParserError::Internal);
      };
      let arguments = if is_contructor {
        parser.get_children(
          &mut LoopArgument::new(
            "function_construct_parameters",
            &[TokenType::Comma],
            &[TokenType::RightParenthesis],
            &[
              (ConstructorParameterParser::test, ConstructorParameterParser::parse),
              (TypesParser::test, TypesParser::parse),
              (AttributeParser::test, AttributeParser::parse),
              (CommentParser::test, CommentParser::parse),
            ]
          )
        )?
      } else {
        FunctionParser::get_parameters(parser)?
      };
      let return_type = FunctionParser::get_return_type(parser).ok();
      let body = if guard!(parser.tokens.get(parser.position)).token_type == TokenType::Semicolon {
        None
      } else {
        Some(BlockParser::new_block(parser)?)
      };
      return Ok(
        FunctionNode::loc(
          !is_ref.is_empty(),
          name,
          arguments,
          return_type,
          body,
          parser.gen_loc(start_loc)
        )
      );
    }
    Err(ParserError::Internal)
  }

  pub fn get_parameters(parser: &mut Parser) -> Result<Vec<Box<Node>>, ParserError> {
    parser.get_children(
      &mut LoopArgument::new(
        "function_parameters",
        &[TokenType::Comma],
        &[TokenType::RightParenthesis],
        &[
          (AttributeParser::test, AttributeParser::parse),
          (CommentParser::test, CommentParser::parse),
          (ParameterParser::test, ParameterParser::parse),
        ]
      )
    )
  }

  pub fn get_return_type(parser: &mut Parser) -> Result<Box<Node>, ParserError> {
    if let Some(next_token) = parser.tokens.get(parser.position) {
      if next_token.token_type == TokenType::Colon {
        parser.position += 1;
        if
          let Some(return_type) = parser.get_statement(
            &mut LoopArgument::new(
              "function_return_type",
              &[TokenType::LeftCurlyBracket, TokenType::Arrow, TokenType::Semicolon],
              &[],
              &[
                (TypesParser::test, TypesParser::parse),
                (CommentParser::test, CommentParser::parse),
              ]
            )
          )?
        {
          return Ok(return_type);
        }
      }
    }
    Err(ParserError::Internal)
  }
}

#[derive(Debug, Clone)]
pub struct ConstructorParameterParser;

impl ConstructorParameterParser {
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
        Lookup::Optional(&[TokenType::BitwiseAnd]),
        Lookup::Optional(&[TokenType::Ellipsis]),
        Lookup::Equal(&[TokenType::Variable]),
        Lookup::Optional(&[TokenType::Assignment]),
      ]
    )
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<LookupResult>,
    start_loc: Location,
    _: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if
      let [modifiers, has_var, prop_type, is_ref, is_variadic, name, has_value] = matched.as_slice()
    {
      let modifiers = cast_lookup_result!(Modifier, &modifiers.wrapper);
      let name = if let LookupResultWrapper::Equal(name) = &name.wrapper {
        IdentifierParser::from_token(name)
      } else {
        return Err(ParserError::Internal);
      };
      let value = if !has_value.is_empty() {
        parser.get_statement(
          &mut LoopArgument::with_tokens(
            "constructor_parameter",
            &[TokenType::Comma, TokenType::RightParenthesis],
            &[]
          )
        )?
      } else {
        None
      };
      let prop_type = cast_lookup_result!(OptionalType, &prop_type.wrapper);
      let item = ParameterNode::loc(
        prop_type.to_owned(),
        !is_ref.is_empty(),
        !is_variadic.is_empty(),
        name,
        value,
        parser.gen_loc(start_loc.clone())
      );
      let mut visibilities = vec![];
      let mut modifier = None;
      if !modifiers.is_empty() {
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
      return Ok(
        ConstructorParameterNode::loc(visibilities, modifier, item, parser.gen_loc(start_loc))
      );
    }
    Err(ParserError::Internal)
  }
}

#[derive(Debug, Clone)]
pub struct ParameterParser;

impl ParameterParser {
  pub fn test(tokens: &[Token], _: &mut LoopArgument) -> Option<Vec<LookupResult>> {
    match_pattern(
      tokens,
      &[
        Lookup::OptionalType,
        Lookup::Optional(&[TokenType::BitwiseAnd]),
        Lookup::Optional(&[TokenType::Ellipsis]),
        Lookup::Equal(&[TokenType::Variable]),
        Lookup::Optional(&[TokenType::Assignment]),
      ]
    )
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<LookupResult>,
    start_loc: Location,
    _: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [prop_type, is_ref, is_ellipsis, name, has_value] = matched.as_slice() {
      let name = if let LookupResultWrapper::Equal(name) = &name.wrapper {
        IdentifierParser::from_token(name)
      } else {
        return Err(ParserError::Internal);
      };
      let value = if !has_value.is_empty() {
        parser.get_statement(
          &mut LoopArgument::with_tokens(
            "parameter",
            &[TokenType::Comma, TokenType::RightParenthesis],
            &[]
          )
        )?
      } else {
        None
      };
      let prop_type = cast_lookup_result!(OptionalType, &prop_type.wrapper);
      return Ok(
        ParameterNode::loc(
          prop_type.to_owned(),
          !is_ref.is_empty(),
          !is_ellipsis.is_empty(),
          name,
          value,
          parser.gen_loc(start_loc)
        )
      );
    }
    Err(ParserError::Internal)
  }
}
