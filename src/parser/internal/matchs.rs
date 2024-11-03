use crate::{
  guard,
  lexer::token::{ Token, TokenType },
  parser::{
    node::Node,
    nodes::matchs::{ MatchArmNode, MatchNode },
    parser::{ Internal, LoopArgument, Parser, ParserInternal },
    utils::{ match_pattern, Lookup },
  },
};

use super::comment::CommentParser;

#[derive(Debug, Clone)]
pub struct MatchParser {}

impl Internal for MatchParser {
  fn test(&self, tokens: &Vec<Token>, _: &LoopArgument) -> Option<Vec<Vec<Token>>> {
    match_pattern(
      tokens,
      [
        Lookup::Equal(vec![TokenType::Match]),
        Lookup::Equal(vec![TokenType::LeftParenthesis]),
      ].to_vec()
    )
  }

  fn parse(&self, parser: &mut Parser, matched: Vec<Vec<Token>>, _: &LoopArgument) -> Option<Node> {
    if let [_, _] = matched.as_slice() {
      let condition = guard!(
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
          &[ParserInternal::MatchArm(MatchArmParser {}), ParserInternal::Comment(CommentParser {})]
        )
      );
      return Some(MatchNode::new(condition, arms));
    }
    None
  }
}

#[derive(Debug, Clone)]
pub struct MatchArmParser {}

impl Internal for MatchArmParser {
  fn test(&self, _: &Vec<Token>, _: &LoopArgument) -> Option<Vec<Vec<Token>>> {
    Some(vec![])
  }

  fn parse(&self, parser: &mut Parser, _: Vec<Vec<Token>>, _: &LoopArgument) -> Option<Node> {
    let conditions = match guard!(parser.tokens.get(parser.position)).token_type {
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
    let body = guard!(
      parser.get_statement(
        &mut LoopArgument::with_tokens("match_arm_body", &[], &[TokenType::Comma])
      )
    );
    parser.position += 1;
    Some(MatchArmNode::new(conditions, body))
  }
}
