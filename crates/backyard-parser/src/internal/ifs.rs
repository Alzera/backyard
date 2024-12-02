use backyard_lexer::token::{ Token, TokenType };
use backyard_nodes::node::{ ElseNode, IfNode, Location, Node };

use crate::{
  error::ParserError,
  guard,
  parser::{ LocationHelper, LoopArgument, Parser },
  utils::{ match_pattern, Lookup },
};

use super::{ block::BlockParser, comment::CommentParser };

#[derive(Debug, Clone)]
pub struct IfParser;

impl IfParser {
  pub fn test(tokens: &[Token], _: &mut LoopArgument) -> Option<Vec<Vec<Token>>> {
    match_pattern(
      tokens,
      &[Lookup::Equal(&[TokenType::If]), Lookup::Equal(&[TokenType::LeftParenthesis])]
    )
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<Vec<Token>>,
    start_loc: Location,
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
      let (is_short, valid) = BlockParser::new_or_short_or_single(
        parser,
        &[TokenType::ElseIf, TokenType::Else, TokenType::EndIf],
        args
      )?;
      if is_short {
        parser.position -= 1;
      }
      let invalid = parser.get_statement(
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
      )?;
      if is_short {
        if let Some(token) = parser.tokens.get(parser.position) {
          if [TokenType::EndIf].contains(&token.token_type) {
            parser.position += 1;
          }
        }
      }
      return Ok(IfNode::new(condition, valid, invalid, is_short, parser.gen_loc(start_loc)));
    }
    Err(ParserError::internal("If", args))
  }
}

#[derive(Debug, Clone)]
pub struct ElseParser;

impl ElseParser {
  pub fn test(tokens: &[Token], _: &mut LoopArgument) -> Option<Vec<Vec<Token>>> {
    match_pattern(tokens, &[Lookup::Equal(&[TokenType::Else, TokenType::ElseIf])])
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<Vec<Token>>,
    start_loc: Location,
    args: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [keyword] = matched.as_slice() {
      if let Some(keyword) = keyword.first() {
        if keyword.token_type == TokenType::ElseIf {
          parser.position += 1;
          let loc = parser.tokens.get(parser.position).unwrap().get_location().unwrap();
          let expr = IfParser::parse(parser, vec![vec![keyword.to_owned()], vec![]], loc, args)?;
          return Ok(ElseNode::new(expr, false, parser.gen_loc(start_loc)));
        }
      }
      if let Some(next_token) = parser.tokens.get(parser.position) {
        if next_token.token_type == TokenType::If {
          parser.position += 2;
          let loc = parser.tokens.get(parser.position).unwrap().get_location().unwrap();
          let expr = IfParser::parse(parser, vec![vec![next_token.to_owned()], vec![]], loc, args)?;
          return Ok(ElseNode::new(expr, false, parser.gen_loc(start_loc)));
        }
      }
      let (is_short, valid) = BlockParser::new_or_short_or_single(
        parser,
        &[TokenType::ElseIf, TokenType::Else, TokenType::EndIf],
        args
      )?;
      return Ok(ElseNode::new(valid, is_short, parser.gen_loc(start_loc)));
    }
    Err(ParserError::internal("Else", args))
  }
}
