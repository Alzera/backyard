use backyard_lexer::token::{ Token, TokenType };
use backyard_nodes::node::{ MatchArmNode, MatchNode, Node };
use utils::guard_none;

use crate::{ parser::{ LoopArgument, Parser }, utils::{ match_pattern, Lookup } };

use super::comment::CommentParser;

#[derive(Debug, Clone)]
pub struct MatchParser {}

impl MatchParser {
  pub fn test(tokens: &Vec<Token>, _: &mut LoopArgument) -> Option<Vec<Vec<Token>>> {
    match_pattern(
      tokens,
      [
        Lookup::Equal(vec![TokenType::Match]),
        Lookup::Equal(vec![TokenType::LeftParenthesis]),
      ].to_vec()
    )
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<Vec<Token>>,
    _: &mut LoopArgument
  ) -> Option<Box<Node>> {
    if let [_, _] = matched.as_slice() {
      let condition = guard_none!(
        parser.get_statement(
          &mut LoopArgument::with_tokens("match", &[], &[TokenType::RightParenthesis])
        )
      );
      parser.position += 2;
      let arms = parser.get_children(
        &mut LoopArgument::new(
          "match_arm",
          &[TokenType::Comma],
          &[TokenType::RightCurlyBracket],
          &[
            (MatchArmParser::test, MatchArmParser::parse),
            (CommentParser::test, CommentParser::parse),
          ]
        )
      );
      return Some(MatchNode::new(condition, arms));
    }
    None
  }
}

#[derive(Debug, Clone)]
pub struct MatchArmParser {}

impl MatchArmParser {
  pub fn test(_: &Vec<Token>, _: &mut LoopArgument) -> Option<Vec<Vec<Token>>> {
    Some(vec![])
  }

  pub fn parse(parser: &mut Parser, _: Vec<Vec<Token>>, _: &mut LoopArgument) -> Option<Box<Node>> {
    let conditions = match guard_none!(parser.tokens.get(parser.position)).token_type {
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
        ),
    };
    let body = guard_none!(
      parser.get_statement(
        &mut LoopArgument::with_tokens("match_arm_body", &[], &[TokenType::Comma])
      )
    );
    parser.position += 1;
    Some(MatchArmNode::new(conditions, body))
  }
}
