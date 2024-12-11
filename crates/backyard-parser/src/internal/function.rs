use bumpalo::{ boxed::Box, collections::Vec, vec };
use backyard_lexer::token::{ Token, TokenType };
use backyard_nodes::{
  node::{
    AnonymousFunctionNode,
    ArrowFunctionNode,
    ConstructorParameterNode,
    FunctionNode,
    Location,
    Modifier,
    Node,
    ParameterNode,
    Visibility,
  },
  utils::{ IntoBoxedNode, IntoBoxedOptionNode },
};

use crate::{
  error::ParserError,
  guard,
  parser::{ LoopArgument, Parser, TokenTypeArrayCombine },
  utils::{ match_pattern, Lookup, LookupResult, LookupResultWrapper, ModifierLookup },
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
  pub fn test<'arena, 'a>(
    parser: &mut Parser<'arena, 'a>,
    tokens: &[Token],
    _: &mut LoopArgument
  ) -> Option<std::vec::Vec<LookupResult<'arena>>> {
    if
      let Some(m) = match_pattern(
        parser,
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
        parser,
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
      parser,
      tokens,
      &[
        Lookup::Equal(&[TokenType::Fn]),
        Lookup::Optional(&[TokenType::BitwiseAnd]),
        Lookup::Equal(&[TokenType::LeftParenthesis]),
      ]
    )
  }

  pub fn parse<'arena, 'a, 'b>(
    parser: &mut Parser<'arena, 'a>,
    matched: std::vec::Vec<LookupResult>,
    start_loc: Location,
    args: &mut LoopArgument<'arena, 'b>
  ) -> Result<Node<'arena>, ParserError> {
    match matched.len() {
      4 => FunctionParser::parse_basic(parser, matched, start_loc),
      3 => {
        if let Some(f) = matched.first() {
          let f = f.as_equal()?;
          if f.token_type == TokenType::Fn {
            return FunctionParser::parse_arrow(parser, matched, start_loc, args);
          } else if f.token_type == TokenType::Function {
            return FunctionParser::parse_anonymous(parser, matched, start_loc);
          }
        }
        Err(ParserError::Internal)
      }
      _ => Err(ParserError::Internal),
    }
  }
}

impl FunctionParser {
  pub fn parse_arrow<'arena, 'a, 'b>(
    parser: &mut Parser<'arena, 'a>,
    matched: std::vec::Vec<LookupResult>,
    start_loc: Location,
    args: &mut LoopArgument<'arena, 'b>
  ) -> Result<Node<'arena>, ParserError> {
    if let [_, is_ref, _] = matched.as_slice() {
      let arguments = FunctionParser::get_parameters(parser)?;
      let return_type = FunctionParser::get_return_type(parser).ok();
      parser.position += 1;
      let body = guard!(
        parser.get_statement(
          &mut LoopArgument::with_tokens(
            parser.arena,
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
          body.into_boxed(&parser.arena),
          parser.gen_loc(start_loc)
        )
      );
    }
    Err(ParserError::Internal)
  }

  pub fn parse_anonymous<'arena, 'a, 'b>(
    parser: &mut Parser<'arena, 'a>,
    matched: std::vec::Vec<LookupResult>,
    start_loc: Location
  ) -> Result<Node<'arena>, ParserError> {
    if let [_, is_ref, _] = matched.as_slice() {
      let arguments = FunctionParser::get_parameters(parser)?;
      let mut uses = vec![in parser.arena];
      if let Some(next_token) = parser.tokens.get(parser.position) {
        if next_token.token_type == TokenType::Use {
          parser.position += 2;
          uses = parser.get_children(
            &mut LoopArgument::with_tokens(
              &parser.arena,
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
          body.into_boxed(&parser.arena),
          parser.gen_loc(start_loc)
        )
      );
    }
    Err(ParserError::Internal)
  }

  pub fn parse_basic<'arena, 'a, 'b>(
    parser: &mut Parser<'arena, 'a>,
    matched: std::vec::Vec<LookupResult>,
    start_loc: Location
  ) -> Result<Node<'arena>, ParserError> {
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
            parser.arena,
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
          name.into_boxed(&parser.arena),
          arguments,
          return_type,
          body.into_boxed(&parser.arena),
          parser.gen_loc(start_loc)
        )
      );
    }
    Err(ParserError::Internal)
  }

  pub fn get_parameters<'arena, 'a>(
    parser: &mut Parser<'arena, 'a>
  ) -> Result<Vec<'arena, Node<'arena>>, ParserError> {
    parser.get_children(
      &mut LoopArgument::new(
        parser.arena,
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

  pub fn get_return_type<'arena, 'a>(
    parser: &mut Parser<'arena, 'a>
  ) -> Result<Box<'arena, Node<'arena>>, ParserError> {
    if let Some(next_token) = parser.tokens.get(parser.position) {
      if next_token.token_type == TokenType::Colon {
        parser.position += 1;
        if
          let Some(return_type) = parser.get_statement(
            &mut LoopArgument::new(
              parser.arena,
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
          return Ok(return_type.into_boxed(&parser.arena));
        }
      }
    }
    Err(ParserError::Internal)
  }
}

#[derive(Debug, Clone)]
pub struct ConstructorParameterParser;

impl ConstructorParameterParser {
  pub fn test<'arena, 'a>(
    parser: &mut Parser<'arena, 'a>,
    tokens: &[Token],
    _: &mut LoopArgument
  ) -> Option<std::vec::Vec<LookupResult<'arena>>> {
    match_pattern(
      parser,
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

  pub fn parse<'arena, 'a, 'b>(
    parser: &mut Parser<'arena, 'a>,
    matched: std::vec::Vec<LookupResult>,
    start_loc: Location,
    _: &mut LoopArgument<'arena, 'b>
  ) -> Result<Node<'arena>, ParserError> {
    if
      let [modifiers, has_var, prop_type, is_ref, is_variadic, name, has_value] = matched.as_slice()
    {
      let value = if !has_value.is_empty() {
        parser.get_statement(
          &mut LoopArgument::with_tokens(
            parser.arena,
            "constructor_parameter",
            &[TokenType::Comma, TokenType::RightParenthesis],
            &[]
          )
        )?
      } else {
        None
      };
      let item = ParameterNode::loc(
        prop_type.as_optional_type(parser.arena).into_boxed(&parser.arena),
        !is_ref.is_empty(),
        !is_variadic.is_empty(),
        IdentifierParser::from_token(name.as_equal()?).into_boxed(&parser.arena),
        value.into_boxed(&parser.arena),
        parser.gen_loc(start_loc.clone())
      );
      let mut visibilities = std::vec![];
      let mut modifier = None;
      if let Some([m0, m1]) = modifiers.as_modifier() {
        visibilities = m0.as_visibilities();
        modifier = m1.as_custom(|x| Modifier::try_from(x));
      }
      if visibilities.is_empty() && !has_var.is_empty() {
        visibilities.push(Visibility::Public);
      }
      return Ok(
        ConstructorParameterNode::loc(
          visibilities,
          modifier,
          item.into_boxed(&parser.arena),
          parser.gen_loc(start_loc)
        )
      );
    }
    Err(ParserError::Internal)
  }
}

#[derive(Debug, Clone)]
pub struct ParameterParser;

impl ParameterParser {
  pub fn test<'arena, 'a>(
    parser: &mut Parser<'arena, 'a>,
    tokens: &[Token],
    _: &mut LoopArgument
  ) -> Option<std::vec::Vec<LookupResult<'arena>>> {
    match_pattern(
      parser,
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

  pub fn parse<'arena, 'a, 'b>(
    parser: &mut Parser<'arena, 'a>,
    matched: std::vec::Vec<LookupResult>,
    start_loc: Location,
    _: &mut LoopArgument<'arena, 'b>
  ) -> Result<Node<'arena>, ParserError> {
    if let [prop_type, is_ref, is_ellipsis, name, has_value] = matched.as_slice() {
      let name = IdentifierParser::from_token(name.as_equal()?);
      let value = if !has_value.is_empty() {
        parser.get_statement(
          &mut LoopArgument::with_tokens(
            parser.arena,
            "parameter",
            &[TokenType::Comma, TokenType::RightParenthesis],
            &[]
          )
        )?
      } else {
        None
      };
      return Ok(
        ParameterNode::loc(
          prop_type.as_optional_type(&parser.arena).into_boxed(&parser.arena),
          !is_ref.is_empty(),
          !is_ellipsis.is_empty(),
          name.into_boxed(&parser.arena),
          value.into_boxed(&parser.arena),
          parser.gen_loc(start_loc)
        )
      );
    }
    Err(ParserError::Internal)
  }
}
