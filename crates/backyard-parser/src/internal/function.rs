use backyard_lexer::token::{ Token, TokenType, TokenTypeArrayCombine };
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
  error::ParserError,
  guard,
  parser::{ LoopArgument, Parser },
  utils::{ match_pattern, Lookup, LookupResult, LookupResultWrapper },
};

use super::{
  attribute::AttributeParser,
  block::BlockParser,
  comment::CommentParser,
  identifier::IdentifierParser,
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
      4 => FunctionParser::parse_basic(parser, matched, start_loc, args),
      3 => {
        if let Some(f) = matched.first() {
          if let LookupResultWrapper::Equal(f) = &f.wrapper {
            if f.token_type == TokenType::Fn {
              return FunctionParser::parse_arrow(parser, matched, start_loc, args);
            } else if f.token_type == TokenType::Function {
              return FunctionParser::parse_anonymous(parser, matched, start_loc, args);
            }
          }
        }
        Err(ParserError::internal("Function parse", args))
      }
      _ => Err(ParserError::internal("Function parse", args)),
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
      let return_type = FunctionParser::get_return_type(parser, args).ok();
      parser.position += 1;
      let body = guard!(
        parser.get_statement(
          &mut LoopArgument::with_tokens(
            "function_arrow",
            &[],
            &args.breakers.combine(args.separators)
          )
        )?,
        {
          return Err(ParserError::internal("ArrowFunction", args));
        }
      );
      return Ok(
        ArrowFunctionNode::new(
          !is_ref.is_empty(),
          arguments,
          return_type,
          body,
          parser.gen_loc(start_loc)
        )
      );
    }
    Err(ParserError::internal("ArrowFunction", args))
  }

  pub fn parse_anonymous(
    parser: &mut Parser,
    matched: Vec<LookupResult>,
    start_loc: Location,
    args: &LoopArgument
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
      let return_type = FunctionParser::get_return_type(parser, args).ok();
      let body = BlockParser::new(parser)?;
      return Ok(
        AnonymousFunctionNode::new(
          !is_ref.is_empty(),
          arguments,
          uses,
          return_type,
          body,
          parser.gen_loc(start_loc)
        )
      );
    }
    Err(ParserError::internal("AnonymousFunction", args))
  }

  pub fn parse_basic(
    parser: &mut Parser,
    matched: Vec<LookupResult>,
    start_loc: Location,
    args: &LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [_, is_ref, name, _] = matched.as_slice() {
      let name = if let LookupResultWrapper::Any(name) = &name.wrapper {
        name
      } else {
        return Err(ParserError::internal("Function parse_basic 1", args));
      };
      let arguments = if name.value == "__construct" {
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
      let return_type = FunctionParser::get_return_type(parser, args).ok();
      let body = if
        guard!(parser.tokens.get(parser.position), {
          return Err(ParserError::internal("Function parse_basic 2", args));
        }).token_type == TokenType::Semicolon
      {
        None
      } else {
        Some(BlockParser::new(parser)?)
      };
      return Ok(
        FunctionNode::new(
          !is_ref.is_empty(),
          IdentifierParser::from_token(name),
          arguments,
          return_type,
          body,
          parser.gen_loc(start_loc)
        )
      );
    }
    Err(ParserError::internal("Function parse_basic 3", args))
  }

  pub fn get_parameters(parser: &mut Parser) -> Result<Vec<Box<Node>>, ParserError> {
    parser.get_children(
      &mut LoopArgument::new(
        "function_parameters",
        &[TokenType::Comma],
        &[TokenType::RightParenthesis],
        &[
          (TypesParser::test, TypesParser::parse),
          (ParameterParser::test, ParameterParser::parse),
          (AttributeParser::test, AttributeParser::parse),
          (CommentParser::test, CommentParser::parse),
        ]
      )
    )
  }

  pub fn get_return_type(
    parser: &mut Parser,
    args: &LoopArgument
  ) -> Result<Box<Node>, ParserError> {
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
    Err(ParserError::internal("Function get_return_type", args))
  }
}

#[derive(Debug, Clone)]
pub struct ConstructorParameterParser;

impl ConstructorParameterParser {
  pub fn test(tokens: &[Token], _: &mut LoopArgument) -> Option<Vec<LookupResult>> {
    println!("\nConstructorParameterParser::test: {:?}", tokens.first());
    if
      let Some(m) = match_pattern(
        tokens,
        &[
          Lookup::Modifiers(
            &[
              &[TokenType::Public, TokenType::Private, TokenType::Protected],
              &[TokenType::Static, TokenType::Readonly],
            ]
          ),
          Lookup::Optional(&[TokenType::Var]),
          Lookup::OptionalType,
          Lookup::Optional(&[TokenType::BitwiseAnd]),
          Lookup::Optional(&[TokenType::Ellipsis]),
          Lookup::Equal(&[TokenType::Variable]),
        ]
      )
    {
      println!("ConstructorParameterParser::test 1");
      return Some(m[..2].to_vec());
    }
    println!("ConstructorParameterParser::test 2");
    None
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<LookupResult>,
    start_loc: Location,
    args: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [modifiers, has_var] = matched.as_slice() {
      let item = guard!(
        parser.get_statement(
          &mut LoopArgument::new(
            "constructor_parameter",
            &[],
            &[TokenType::Comma, TokenType::RightParenthesis],
            &[
              (AttributeParser::test, AttributeParser::parse),
              (CommentParser::test, CommentParser::parse),
              (TypesParser::test, TypesParser::parse),
              (ParameterParser::test, ParameterParser::parse),
            ]
          )
        )?,
        {
          return Err(ParserError::internal("ConstructorParameter", args));
        }
      );
      let mut visibility = None;
      let mut modifier = None;
      if let LookupResultWrapper::Modifier(modifiers) = &modifiers.wrapper {
        if let [visibility_modifier, modifier_modifier] = modifiers.as_slice() {
          visibility = Visibility::try_parse(
            &visibility_modifier
              .as_ref()
              .map(|i| i.value.to_owned())
              .unwrap_or_default()
          );
          modifier = Modifier::try_parse(
            &modifier_modifier
              .as_ref()
              .map(|i| i.value.to_owned())
              .unwrap_or_default()
          );
        }
      }
      if visibility.is_none() && !has_var.is_empty() {
        visibility = Some(Visibility::Public);
      }
      return Ok(
        ConstructorParameterNode::new(visibility, modifier, item, parser.gen_loc(start_loc))
      );
    }
    Err(ParserError::internal("ConstructorParameter", args))
  }
}

#[derive(Debug, Clone)]
pub struct ParameterParser;

impl ParameterParser {
  pub fn test(tokens: &[Token], _: &mut LoopArgument) -> Option<Vec<LookupResult>> {
    match_pattern(
      tokens,
      &[
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
    args: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [is_ref, is_ellipsis, name, has_value] = matched.as_slice() {
      let name = if let LookupResultWrapper::Equal(name) = &name.wrapper {
        IdentifierParser::from_token(name)
      } else {
        return Err(ParserError::internal("Parameter", args));
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
      let is_ref = !is_ref.is_empty();
      let is_ellipsis = !is_ellipsis.is_empty();
      return Ok(
        ParameterNode::new(
          args.last_expr.to_owned(),
          is_ref,
          is_ellipsis,
          name,
          value,
          parser.gen_loc(start_loc)
        )
      );
    }
    Err(ParserError::internal("Parameter", args))
  }
}
