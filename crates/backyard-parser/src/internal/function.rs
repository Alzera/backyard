use backyard_lexer::token::{ Token, TokenType, TokenTypeArrayCombine };
use backyard_nodes::node::{
  AnonymousFunctionNode,
  ArrowFunctionNode,
  FunctionNode,
  Location,
  Modifier,
  Node,
  ParameterNode,
  PropertyNode,
  Visibility,
};

use crate::{
  error::ParserError,
  guard,
  parser::{ LoopArgument, Parser },
  utils::{ match_pattern, Lookup },
};

use super::{
  attribute::AttributeParser,
  block::BlockParser,
  comment::CommentParser,
  identifier::IdentifierParser,
  property::{ PropertyItemParser, PropertyParser },
  types::TypesParser,
};

#[derive(Debug, Clone)]
pub struct FunctionParser;

impl FunctionParser {
  pub fn class_test(tokens: &[Token], _: &mut LoopArgument) -> Option<Vec<Vec<Token>>> {
    match_pattern(
      tokens,
      &[
        Lookup::Equal(&[TokenType::Function]),
        Lookup::Optional(&[TokenType::BitwiseAnd]),
        Lookup::Any,
        Lookup::Equal(&[TokenType::LeftParenthesis]),
      ]
    )
  }

  pub fn test(tokens: &[Token], _: &mut LoopArgument) -> Option<Vec<Vec<Token>>> {
    if
      let Some(m) = match_pattern(
        tokens,
        &[
          Lookup::Equal(&[TokenType::Function]),
          Lookup::Optional(&[TokenType::BitwiseAnd]),
          Lookup::Equal(&[TokenType::Identifier, TokenType::Readonly]),
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
    matched: Vec<Vec<Token>>,
    start_loc: Location,
    args: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    match matched.len() {
      4 => FunctionParser::parse_basic(parser, matched, start_loc, args),
      3 => {
        if let Some(f) = matched.first() {
          if let Some(f) = f.first() {
            if f.token_type == TokenType::Fn {
              return FunctionParser::parse_arrow(parser, matched, start_loc, args);
            } else if f.token_type == TokenType::Function {
              return FunctionParser::parse_anonymous(parser, matched, start_loc, args);
            }
          }
        }
        Err(ParserError::internal("Function", args))
      }
      _ => Err(ParserError::internal("Function", args)),
    }
  }
}

impl FunctionParser {
  pub fn parse_arrow(
    parser: &mut Parser,
    matched: Vec<Vec<Token>>,
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
    matched: Vec<Vec<Token>>,
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
    matched: Vec<Vec<Token>>,
    start_loc: Location,
    args: &LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [_, is_ref, name, _] = matched.as_slice() {
      let name_parsed = name
        .first()
        .map(|i| i.value.to_owned())
        .unwrap_or_default();
      let arguments = if name_parsed == "__construct" {
        parser.get_children(
          &mut LoopArgument::new(
            "function_construct_parameters",
            &[TokenType::Comma],
            &[TokenType::RightParenthesis],
            &[
              (ConstructorParameterParser::test, ConstructorParameterParser::parse),
              (TypesParser::test, TypesParser::parse),
              (ParameterParser::test, ParameterParser::parse),
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
          return Err(ParserError::internal("Function", args));
        }).token_type == TokenType::Semicolon
      {
        None
      } else {
        Some(BlockParser::new(parser)?)
      };
      return Ok(
        FunctionNode::new(
          !is_ref.is_empty(),
          IdentifierParser::from_matched(name),
          arguments,
          return_type,
          body,
          parser.gen_loc(start_loc)
        )
      );
    }
    Err(ParserError::internal("Function", args))
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
    Err(ParserError::internal("Function", args))
  }
}

#[derive(Debug, Clone)]
pub struct ConstructorParameterParser;

impl ConstructorParameterParser {
  pub fn test(tokens: &[Token], args: &mut LoopArgument) -> Option<Vec<Vec<Token>>> {
    PropertyParser::test(tokens, args)
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<Vec<Token>>,
    start_loc: Location,
    args: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [visibility, modifier, has_var] = matched.as_slice() {
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
              (PropertyItemParser::test, PropertyItemParser::parse),
            ]
          )
        )?,
        {
          return Err(ParserError::internal("ConstructorParameter", args));
        }
      );
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
          vec![item],
          parser.gen_loc(start_loc)
        )
      );
    }
    Err(ParserError::internal("ConstructorParameter", args))
  }
}

#[derive(Debug, Clone)]
pub struct ParameterParser;

impl ParameterParser {
  pub fn test(tokens: &[Token], _: &mut LoopArgument) -> Option<Vec<Vec<Token>>> {
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
    matched: Vec<Vec<Token>>,
    start_loc: Location,
    args: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [is_ref, is_ellipsis, name, has_value] = matched.as_slice() {
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
          IdentifierParser::from_matched(name),
          value,
          parser.gen_loc(start_loc)
        )
      );
    }
    Err(ParserError::internal("Parameter", args))
  }
}
