use backyard_lexer::token::{ Token, TokenType };
use backyard_nodes::node::{ CatchNode, FinallyNode, Node, TryNode };

use crate::{
  error::ParserError,
  parser::{ LoopArgument, Parser },
  utils::{ match_pattern, Lookup },
};

use super::{
  block::BlockParser,
  comment::CommentParser,
  identifier::IdentifierParser,
  variable::VariableParser,
};

#[derive(Debug, Clone)]
pub struct TryParser {}

impl TryParser {
  pub fn test(tokens: &Vec<Token>, _: &mut LoopArgument) -> Option<Vec<Vec<Token>>> {
    match_pattern(tokens, [Lookup::Equal(vec![TokenType::Try])].to_vec())
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<Vec<Token>>,
    args: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [_] = matched.as_slice() {
      let body = BlockParser::new(parser)?;
      let catches = parser.get_children(
        &mut LoopArgument::safe(
          "try",
          &[],
          &[],
          &[
            (FinallyParser::test, FinallyParser::parse),
            (CatchParser::test, CatchParser::parse),
            (CommentParser::test, CommentParser::parse),
          ]
        )
      )?;
      return Ok(TryNode::new(body, catches));
    }
    Err(ParserError::internal("Try", args))
  }
}

#[derive(Debug, Clone)]
pub struct CatchParser {}

impl CatchParser {
  pub fn test(tokens: &Vec<Token>, _: &mut LoopArgument) -> Option<Vec<Vec<Token>>> {
    match_pattern(
      tokens,
      [
        Lookup::Equal(vec![TokenType::Catch]),
        Lookup::Equal(vec![TokenType::LeftParenthesis]),
      ].to_vec()
    )
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<Vec<Token>>,
    args: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [_, _] = matched.as_slice() {
      let types = parser.get_children(
        &mut LoopArgument::new(
          "catch_types",
          &[TokenType::BitwiseOr],
          &[TokenType::Variable, TokenType::VariableBracketOpen, TokenType::RightParenthesis],
          &[
            (IdentifierParser::test, IdentifierParser::parse),
            (CommentParser::test, CommentParser::parse),
          ]
        )
      )?;
      parser.position -= 1;
      let mut variable = None;
      if let Some(last_token) = parser.tokens.get(parser.position) {
        if last_token.token_type != TokenType::RightParenthesis {
          variable = parser.get_statement(
            &mut LoopArgument::new(
              "catch_variable",
              &[],
              &[TokenType::RightParenthesis],
              &[
                (VariableParser::test, VariableParser::parse),
                (CommentParser::test, CommentParser::parse),
              ]
            )
          )?;
        }
      }
      parser.position += 1;
      let body = BlockParser::new(parser)?;
      return Ok(CatchNode::new(types, variable, body));
    }
    Err(ParserError::internal("DoWhileCondition", args))
  }
}

#[derive(Debug, Clone)]
pub struct FinallyParser {}

impl FinallyParser {
  pub fn test(tokens: &Vec<Token>, _: &mut LoopArgument) -> Option<Vec<Vec<Token>>> {
    match_pattern(tokens, [Lookup::Equal(vec![TokenType::Finally])].to_vec())
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<Vec<Token>>,
    args: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [_] = matched.as_slice() {
      let body = BlockParser::new(parser)?;
      return Ok(FinallyNode::new(body));
    }
    Err(ParserError::internal("Finally", args))
  }
}
