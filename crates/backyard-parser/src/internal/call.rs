use std::vec;

use backyard_lexer::token::{ Token, TokenType };
use backyard_nodes::node::{ ArgumentNode, CallNode, Node };
use utils::guard;

use crate::{ error::ParserError, parser::{ LoopArgument, Parser } };

use super::{ comment::CommentParser, identifier::IdentifierParser };

#[derive(Debug, Clone)]
pub struct CallParser;

impl CallParser {
  pub fn get_arguments(parser: &mut Parser) -> Result<Vec<Box<Node>>, ParserError> {
    parser.get_children(
      &mut LoopArgument::new(
        "call",
        &[TokenType::Comma],
        &[TokenType::RightParenthesis],
        &[
          (CommentParser::test, CommentParser::parse),
          (ArgumentParser::test, ArgumentParser::parse),
        ]
      )
    )
  }
}

impl CallParser {
  pub fn test(tokens: &[Token], args: &mut LoopArgument) -> Option<Vec<Vec<Token>>> {
    if args.last_expr.is_some() {
      if let Some(next_token) = tokens.first() {
        if next_token.token_type == TokenType::LeftParenthesis {
          return Some(vec![vec![next_token.to_owned()]]);
        }
      }
    }
    None
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<Vec<Token>>,
    args: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [_] = matched.as_slice() {
      return Ok(
        CallNode::new(args.last_expr.to_owned().unwrap(), CallParser::get_arguments(parser)?)
      );
    }
    Err(ParserError::internal("Call", args))
  }
}

#[derive(Debug, Clone)]
pub struct ArgumentParser;

impl ArgumentParser {
  pub fn test(tokens: &[Token], _: &mut LoopArgument) -> Option<Vec<Vec<Token>>> {
    if let Some(is_colon) = tokens.get(1) {
      if is_colon.token_type == TokenType::Colon {
        if let Some(name) = tokens.first() {
          return Some(vec![vec![name.to_owned()], vec![is_colon.to_owned()]]);
        }
      }
    }
    Some(vec![vec![], vec![]])
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<Vec<Token>>,
    args: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [name, has_name] = matched.as_slice() {
      let name = if !has_name.is_empty() {
        Some(IdentifierParser::from_matched(name))
      } else {
        parser.position -= name.len();
        None
      };
      let value = guard!(
        parser.get_statement(
          &mut LoopArgument::with_tokens(
            "argument",
            &[TokenType::Comma, TokenType::RightParenthesis],
            &[]
          )
        )?,
        {
          return Err(ParserError::internal("Argument: failed to get value", args));
        }
      );
      return Ok(ArgumentNode::new(name, value));
    }
    Err(ParserError::internal("Argument", args))
  }
}
