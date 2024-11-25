use backyard_lexer::token::{ Token, TokenType };
use backyard_nodes::node::{ ElseNode, IfNode, Node };
use utils::guard;

use crate::{
  error::ParserError,
  parser::{ LoopArgument, Parser },
  utils::{ match_pattern, Lookup },
};

use super::{ block::BlockParser, comment::CommentParser };

#[derive(Debug, Clone)]
pub struct IfParser {}

impl IfParser {
  pub fn test(tokens: &Vec<Token>, _: &mut LoopArgument) -> Option<Vec<Vec<Token>>> {
    match_pattern(
      tokens,
      [Lookup::Equal(vec![TokenType::If]), Lookup::Equal(vec![TokenType::LeftParenthesis])].to_vec()
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
          &mut LoopArgument::with_tokens("if", &[], &[TokenType::RightParenthesis])
        )?,
        {
          return Err(ParserError::internal("If", args));
        }
      );
      parser.position += 1;
      let (is_short, valid) = BlockParser::new_or_short(
        parser,
        &[TokenType::ElseIf, TokenType::Else, TokenType::EndIf], // }
        &mut LoopArgument::default("if_valid")
      )?;
      if let Some(token) = parser.tokens.get(parser.position - 1) {
        if token.token_type != TokenType::RightCurlyBracket {
          parser.position -= 1;
        }
      }
      let invalid = if
        let Some(i) = parser.get_statement(
          &mut LoopArgument::safe(
            "if_invalid",
            &[],
            &[TokenType::RightCurlyBracket, TokenType::EndIf],
            &[
              // (IfParser::test, IfParser::parse),
              (ElseParser::test, ElseParser::parse),
              (CommentParser::test, CommentParser::parse),
            ]
          )
        )?
      {
        if let Some(token) = parser.tokens.get(parser.position) {
          if [TokenType::EndIf].contains(&token.token_type) {
            parser.position += 1;
          }
        }
        Some(i)
      } else {
        None
      };
      return Ok(IfNode::new(condition, valid, invalid, is_short));
    }
    Err(ParserError::internal("If", args))
  }
}

#[derive(Debug, Clone)]
pub struct ElseParser {}

impl ElseParser {
  pub fn test(tokens: &Vec<Token>, _: &mut LoopArgument) -> Option<Vec<Vec<Token>>> {
    match_pattern(tokens, [Lookup::Equal(vec![TokenType::Else, TokenType::ElseIf])].to_vec())
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<Vec<Token>>,
    args: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [keyword] = matched.as_slice() {
      if let Some(keyword) = keyword.get(0) {
        if keyword.token_type == TokenType::ElseIf {
          parser.position += 1;
          let expr = IfParser::parse(parser, vec![vec![keyword.to_owned()], vec![]], args)?;
          return Ok(ElseNode::new(expr, false));
        }
      }
      let (is_short, valid) = BlockParser::new_or_short(
        parser,
        &[TokenType::EndIf],
        &mut LoopArgument::default("else")
      )?;
      // parser.position -= 1;
      return Ok(ElseNode::new(valid, is_short));
    }
    Err(ParserError::internal("Else", args))
  }
}
