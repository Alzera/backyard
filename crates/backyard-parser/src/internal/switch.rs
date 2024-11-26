use backyard_lexer::token::{ Token, TokenType };
use backyard_nodes::node::{ BlockNode, CaseNode, Node, SwitchNode };
use utils::guard;

use crate::{
  error::ParserError,
  parser::{ LoopArgument, Parser },
  utils::{ match_pattern, Lookup },
};

use super::{ block::BlockParser, comment::CommentParser };

#[derive(Debug, Clone)]
pub struct SwitchParser;

impl SwitchParser {
  pub fn test(tokens: &[Token], _: &mut LoopArgument) -> Option<Vec<Vec<Token>>> {
    match_pattern(
      tokens,
      &[Lookup::Equal(&[TokenType::Switch]), Lookup::Equal(&[TokenType::LeftParenthesis])]
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
          &mut LoopArgument::with_tokens("switch", &[], &[TokenType::RightParenthesis])
        )?,
        {
          return Err(ParserError::internal("Switch", args));
        }
      );
      parser.position += 1;
      let is_short =
        guard!(parser.tokens.get(parser.position), {
          return Err(ParserError::internal("Switch", args));
        }).token_type == TokenType::Colon;
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
      )?;
      return Ok(SwitchNode::new(condition, BlockNode::new(statements), is_short));
    }
    Err(ParserError::internal("Switch", args))
  }
}

#[derive(Debug, Clone)]
pub struct CaseParser;

impl CaseParser {
  pub fn test(tokens: &[Token], _: &mut LoopArgument) -> Option<Vec<Vec<Token>>> {
    match_pattern(tokens, &[Lookup::Equal(&[TokenType::Case, TokenType::Default])])
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<Vec<Token>>,
    args: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [is_default] = matched.as_slice() {
      let condition = match
        guard!(is_default.first(), {
          return Err(ParserError::internal("Case", args));
        }).token_type
      {
        TokenType::Default => None,
        _ => {
          parser.get_statement(
            &mut LoopArgument::with_tokens("switch_case_condition", &[], &[TokenType::Colon])
          )?
        }
      };
      parser.position += 1;
      let statements = {
        let token = guard!(parser.tokens.get(parser.position), {
          return Err(ParserError::internal("Switch", args));
        }).token_type;
        if token == TokenType::LeftCurlyBracket {
          BlockParser::new(parser)?
        } else {
          let s = parser.get_children(
            &mut LoopArgument::with_tokens(
              "switch_case_body",
              &[TokenType::Semicolon],
              &[
                TokenType::Case,
                TokenType::Default,
                TokenType::RightCurlyBracket,
                TokenType::EndSwitch,
              ]
            )
          )?;
          parser.position -= 1;
          BlockNode::new(s)
        }
      };
      return Ok(CaseNode::new(condition, statements));
    }
    Err(ParserError::internal("Case", args))
  }
}
