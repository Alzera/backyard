use crate::{
  guard_none,
  lexer::token::{ Token, TokenType },
  parser::{
    node::Node,
    nodes::{ block::BlockNode, switch::{ CaseNode, SwitchNode } },
    parser::{ LoopArgument, Parser },
    utils::{ match_pattern, Lookup },
  },
};

use super::comment::CommentParser;

#[derive(Debug, Clone)]
pub struct SwitchParser {}

impl SwitchParser {
  pub fn test(tokens: &Vec<Token>, _: &mut LoopArgument) -> Option<Vec<Vec<Token>>> {
    match_pattern(
      tokens,
      [
        Lookup::Equal(vec![TokenType::Switch]),
        Lookup::Equal(vec![TokenType::LeftParenthesis]),
      ].to_vec()
    )
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<Vec<Token>>,
    _: &mut LoopArgument
  ) -> Option<Node> {
    if let [_, _] = matched.as_slice() {
      let condition = guard_none!(
        parser.get_statement(
          &mut LoopArgument::with_tokens("switch", &[], &[TokenType::RightParenthesis])
        )
      );
      parser.position += 1;
      let is_short = guard_none!(parser.tokens.get(parser.position)).token_type == TokenType::Colon;
      parser.position += 1;
      let statements = parser.get_children(
        &mut LoopArgument::new(
          "switch_body",
          &[],
          &[TokenType::RightCurlyBracket, TokenType::EndSwitch],
          &[
            (CaseParser::test, CaseParser::parse),
            (CommentParser::test, CommentParser::parse),
          ]
        )
      );
      return Some(SwitchNode::new(condition, BlockNode::new(statements), is_short));
    }
    None
  }
}

#[derive(Debug, Clone)]
pub struct CaseParser {}

impl CaseParser {
  pub fn test(tokens: &Vec<Token>, _: &mut LoopArgument) -> Option<Vec<Vec<Token>>> {
    match_pattern(tokens, [Lookup::Equal(vec![TokenType::Case, TokenType::Default])].to_vec())
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<Vec<Token>>,
    _: &mut LoopArgument
  ) -> Option<Node> {
    if let [is_default] = matched.as_slice() {
      let condition = match guard_none!(is_default.get(0)).token_type {
        TokenType::Default => None,
        _ => {
          parser.get_statement(
            &mut LoopArgument::with_tokens("switch_case_condition", &[], &[TokenType::Colon])
          )
        }
      };
      parser.position += 1;
      let statements = parser.get_children(
        &mut LoopArgument::with_tokens(
          "switch_case_body",
          &[TokenType::Semicolon],
          &[TokenType::Case, TokenType::Default, TokenType::RightCurlyBracket, TokenType::EndSwitch]
        )
      );
      parser.position -= 1;
      return Some(CaseNode::new(condition, BlockNode::new(statements)));
    }
    None
  }
}
