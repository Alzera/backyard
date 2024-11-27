use backyard_lexer::token::{ Token, TokenType };
use backyard_nodes::node::{ MatchArmNode, MatchNode, Node };

use crate::{
  error::ParserError,
  guard,
  parser::{ LoopArgument, Parser },
  utils::{ match_pattern, Lookup },
};

use super::comment::CommentParser;

#[derive(Debug, Clone)]
pub struct MatchParser;

impl MatchParser {
  pub fn test(tokens: &[Token], _: &mut LoopArgument) -> Option<Vec<Vec<Token>>> {
    match_pattern(
      tokens,
      &[Lookup::Equal(&[TokenType::Match]), Lookup::Equal(&[TokenType::LeftParenthesis])]
    )
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<Vec<Token>>,
    args: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [_, _] = matched.as_slice() {
      let condition = guard!(
        parser.get_statement(
          &mut LoopArgument::with_tokens("match", &[], &[TokenType::RightParenthesis])
        )?,
        {
          return Err(ParserError::internal("Match", args));
        }
      );
      parser.position += 2;
      let arms = parser.get_children(
        &mut LoopArgument::new(
          "match_arm",
          &[TokenType::Comma],
          &[TokenType::RightCurlyBracket],
          &[
            (CommentParser::test, CommentParser::parse),
            (MatchArmParser::test, MatchArmParser::parse),
          ]
        )
      )?;
      return Ok(MatchNode::new(condition, arms));
    }
    Err(ParserError::internal("Match", args))
  }
}

#[derive(Debug, Clone)]
pub struct MatchArmParser;

impl MatchArmParser {
  pub fn test(_: &[Token], _: &mut LoopArgument) -> Option<Vec<Vec<Token>>> {
    Some(vec![])
  }

  pub fn parse(
    parser: &mut Parser,
    _: Vec<Vec<Token>>,
    args: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    let conditions = match
      guard!(parser.tokens.get(parser.position), {
        return Err(ParserError::internal("MatchArm", args));
      }).token_type
    {
      TokenType::Default => {
        parser.position += 2;
        vec![]
      }
      _ =>
        parser.get_children(
          &mut LoopArgument::with_tokens(
            "match_arm_condition",
            &[TokenType::Comma],
            &[TokenType::Arrow]
          )
        )?,
    };
    let body = guard!(
      parser.get_statement(
        &mut LoopArgument::with_tokens(
          "match_arm_body",
          &[],
          &[TokenType::Comma, TokenType::RightCurlyBracket]
        )
      )?,
      {
        return Err(ParserError::internal("MatchArm", args));
      }
    );
    if let Some(next_token) = parser.tokens.get(parser.position) {
      if next_token.token_type == TokenType::Comma {
        parser.position += 1;
      }
    }
    Ok(MatchArmNode::new(conditions, body))
  }
}
