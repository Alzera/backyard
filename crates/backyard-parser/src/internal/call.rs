use backyard_lexer::token::{ Token, TokenType };
use backyard_nodes::node::{ ArgumentNode, CallNode, Node };
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
  pub fn test(tokens: &Vec<Token>, args: &mut LoopArgument) -> Option<Vec<Vec<Token>>> {
    if let Some(_) = &args.last_expr {
      if let Some(next_token) = tokens.get(0) {
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
pub struct ArgumentParser {}

impl ArgumentParser {
  pub fn test(tokens: &Vec<Token>, _: &mut LoopArgument) -> Option<Vec<Vec<Token>>> {
    match_pattern(
      tokens,
      [
        Lookup::Optional(vec![TokenType::Identifier, TokenType::Default]),
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
