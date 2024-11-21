use backyard_lexer::token::{ Token, TokenType };
use backyard_nodes::node::{ ArgumentNode, CallNode, Node, NodeType };
use utils::guard;

use crate::{
  error::ParserError,
  parser::{ LoopArgument, Parser },
  utils::{ match_pattern, Lookup },
};

use super::{ comment::CommentParser, identifier::IdentifierParser };

#[derive(Debug, Clone)]
pub struct CallParser {}

impl CallParser {
  pub fn get_arguments(parser: &mut Parser) -> Result<Vec<Box<Node>>, ParserError> {
    parser.get_children(
      &mut LoopArgument::new(
        "call",
        &[TokenType::Comma],
        &[TokenType::RightParenthesis],
        &[
          (ArgumentParser::test, ArgumentParser::parse),
          (CommentParser::test, CommentParser::parse),
        ]
      )
    )
  }
}

impl CallParser {
  pub fn class_test(tokens: &Vec<Token>, _: &mut LoopArgument) -> Option<Vec<Vec<Token>>> {
    match_pattern(
      tokens,
      [
        Lookup::Optional(vec![TokenType::BackSlash]),
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
            TokenType::From
          ]
        ),
        Lookup::Equal(vec![TokenType::LeftParenthesis]),
      ].to_vec()
    )
  }

  pub fn test(tokens: &Vec<Token>, args: &mut LoopArgument) -> Option<Vec<Vec<Token>>> {
    if let Some(last_expr) = &args.last_expr {
      if [NodeType::Variable, NodeType::ObjectAccess].contains(&last_expr.node_type) {
        if let Some(next_token) = tokens.get(0) {
          if next_token.token_type == TokenType::LeftParenthesis {
            return Some(vec![vec![next_token.to_owned()]]);
          }
        }
      }
    }
    match_pattern(
      tokens,
      [
        Lookup::Optional(vec![TokenType::BackSlash]),
        Lookup::Equal(vec![TokenType::Identifier]),
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
      1 => {
        if let [_] = matched.as_slice() {
          return Ok(
            CallNode::new(args.last_expr.to_owned().unwrap(), CallParser::get_arguments(parser)?)
          );
        }
      }
      3 => {
        if let [backslash, name, _] = matched.as_slice() {
          if let Some(name) = name.get(0) {
            let name = if let Some(_) = backslash.get(0) {
              format!("\\{}", name.value.to_owned())
            } else {
              name.value.to_owned()
            };
            return Ok(
              CallNode::new(IdentifierParser::new(name), CallParser::get_arguments(parser)?)
            );
          }
        }
      }
      _ => {
        return Err(ParserError::internal("Call", args));
      }
    }
    Err(ParserError::internal("Call", args))
  }
}

#[derive(Debug, Clone)]
pub struct ArgumentParser {}

impl ArgumentParser {
  pub fn test(tokens: &Vec<Token>, _: &mut LoopArgument) -> Option<Vec<Vec<Token>>> {
    match_pattern(
      tokens,
      [
        Lookup::Optional(vec![TokenType::Identifier]),
        Lookup::Optional(vec![TokenType::Colon]),
      ].to_vec()
    )
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<Vec<Token>>,
    args: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [name, has_name] = matched.as_slice() {
      let name = if has_name.len() > 0 {
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
