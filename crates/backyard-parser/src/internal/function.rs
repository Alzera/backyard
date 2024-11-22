use backyard_lexer::token::{ Token, TokenType, TokenTypeArrayCombine };
use backyard_nodes::node::{
  Node,
  AnonymousFunctionNode,
  ArrowFunctionNode,
  FunctionNode,
  ParameterNode,
  PropertyNode,
};
use utils::guard;

use crate::{
  error::ParserError,
  parser::{ LoopArgument, Parser },
  utils::{ match_pattern, some_or_default, Lookup },
};

use super::{
  block::BlockParser,
  comment::CommentParser,
  identifier::IdentifierParser,
  property::{ PropertyItemParser, PropertyParser },
  types::TypesParser,
};

#[derive(Debug, Clone)]
pub struct FunctionParser {}

impl FunctionParser {
  pub fn class_test(tokens: &Vec<Token>, _: &mut LoopArgument) -> Option<Vec<Vec<Token>>> {
    match_pattern(
      tokens,
      [
        Lookup::Equal(vec![TokenType::Function]),
        Lookup::Optional(vec![TokenType::BitwiseAnd]),
        Lookup::Equal(
          vec![
            TokenType::Identifier,
            TokenType::Clone,
            TokenType::Echo,
            TokenType::For,
            TokenType::If,
            TokenType::While,
            TokenType::Array,
            TokenType::List,
            TokenType::Global,
            TokenType::Print,
            TokenType::Type,
            TokenType::From,
            TokenType::And,
            TokenType::Or,
            TokenType::Xor,
            TokenType::New,
            TokenType::Default,
            TokenType::Class,
            TokenType::Callable,
            TokenType::Throw,
            TokenType::Use,
            TokenType::Match,
            TokenType::Eval,
            TokenType::Catch,
            TokenType::Parent,
            TokenType::Namespace,
            TokenType::As,
            TokenType::Static,
            TokenType::Trait,
            TokenType::Function,
            TokenType::Extends,
            TokenType::Implements,
            TokenType::Var,
            TokenType::Else,
            TokenType::Finally,
            TokenType::Final,
            TokenType::InsteadOf,
            TokenType::SelfKeyword
          ]
        ),
        Lookup::Equal(vec![TokenType::LeftParenthesis]),
      ].to_vec()
    )
  }

  pub fn test(tokens: &Vec<Token>, _: &mut LoopArgument) -> Option<Vec<Vec<Token>>> {
    if
      let Some(m) = match_pattern(
        tokens,
        [
          Lookup::Equal(vec![TokenType::Function]),
          Lookup::Optional(vec![TokenType::BitwiseAnd]),
          Lookup::Equal(vec![TokenType::Identifier]),
          Lookup::Equal(vec![TokenType::LeftParenthesis]),
        ].to_vec()
      )
    {
      return Some(m);
    }
    if
      let Some(m) = match_pattern(
        tokens,
        [
          Lookup::Equal(vec![TokenType::Function]),
          Lookup::Optional(vec![TokenType::BitwiseAnd]),
          Lookup::Equal(vec![TokenType::LeftParenthesis]),
        ].to_vec()
      )
    {
      return Some(m);
    }
    match_pattern(
      tokens,
      [
        Lookup::Equal(vec![TokenType::Fn]),
        Lookup::Optional(vec![TokenType::BitwiseAnd]),
        Lookup::Equal(vec![TokenType::LeftParenthesis]),
      ].to_vec()
    )
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<Vec<Token>>,
    args: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    match matched.len() {
      4 => FunctionParser::parse_basic(matched, parser, args),
      3 => {
        if let Some(f) = matched.get(0) {
          if let Some(f) = f.get(0) {
            if f.token_type == TokenType::Fn {
              return FunctionParser::parse_arrow(matched, parser, args);
            } else if f.token_type == TokenType::Function {
              return FunctionParser::parse_anonymous(matched, parser, args);
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
    matched: Vec<Vec<Token>>,
    parser: &mut Parser,
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
      return Ok(ArrowFunctionNode::new(is_ref.len() > 0, arguments, return_type, body));
    }
    Err(ParserError::internal("ArrowFunction", args))
  }

  pub fn parse_anonymous(
    matched: Vec<Vec<Token>>,
    parser: &mut Parser,
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
      return Ok(AnonymousFunctionNode::new(is_ref.len() > 0, arguments, uses, return_type, body));
    }
    Err(ParserError::internal("AnonymousFunction", args))
  }

  pub fn parse_basic(
    matched: Vec<Vec<Token>>,
    parser: &mut Parser,
    args: &LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [_, is_ref, name, _] = matched.as_slice() {
      let name = some_or_default(name.get(0), String::from(""), |i| i.value.to_owned());
      let arguments = if name == "__construct" {
        parser.get_children(
          &mut LoopArgument::new(
            "function_construct_parameters",
            &[TokenType::Comma],
            &[TokenType::RightParenthesis],
            &[
              (ConstructorParameterParser::test, ConstructorParameterParser::parse),
              (TypesParser::test, TypesParser::parse),
              (ParameterParser::test, ParameterParser::parse),
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
          is_ref.len() > 0,
          IdentifierParser::new(name),
          arguments,
          return_type,
          body
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
pub struct ConstructorParameterParser {}

impl ConstructorParameterParser {
  pub fn test(tokens: &Vec<Token>, args: &mut LoopArgument) -> Option<Vec<Vec<Token>>> {
    PropertyParser::test(tokens, args)
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<Vec<Token>>,
    args: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [visibility, modifier] = matched.as_slice() {
      let item = guard!(
        parser.get_statement(
          &mut LoopArgument::new(
            "constructor_parameter",
            &[],
            &[TokenType::Comma, TokenType::RightParenthesis],
            &[
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
      return Ok(
        PropertyNode::new(
          some_or_default(visibility.get(0), String::from(""), |i| i.value.to_owned()),
          some_or_default(modifier.get(0), String::from(""), |i| i.value.to_owned()),
          vec![item]
        )
      );
    }
    Err(ParserError::internal("ConstructorParameter", args))
  }
}

#[derive(Debug, Clone)]
pub struct ParameterParser {}

impl ParameterParser {
  pub fn test(tokens: &Vec<Token>, _: &mut LoopArgument) -> Option<Vec<Vec<Token>>> {
    match_pattern(
      tokens,
      [
        Lookup::Optional(vec![TokenType::Reference, TokenType::Ellipsis]),
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
    if let [is_ref_or_ellipsis, name, has_value] = matched.as_slice() {
      let value = if has_value.len() > 0 {
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
      let mut is_ref = false;
      let mut is_ellipsis = false;
      if let Some(t) = is_ref_or_ellipsis.get(0) {
        match t.token_type {
          TokenType::Reference => {
            is_ref = true;
          }
          TokenType::Ellipsis => {
            is_ellipsis = true;
          }
          _ => {}
        }
      }
      return Ok(
        ParameterNode::new(
          args.last_expr.to_owned(),
          is_ref,
          is_ellipsis,
          IdentifierParser::from_matched(name),
          value
        )
      );
    }
    Err(ParserError::internal("Parameter", args))
  }
}
