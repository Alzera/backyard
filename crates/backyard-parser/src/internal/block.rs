use backyard_lexer::token::{ Token, TokenType };
use backyard_nodes::node::{ BlockNode, Location, Node };

use crate::{
  error::ParserError,
  guard,
  parser::{ LocationHelper, LoopArgument, Parser, DEFAULT_PARSERS },
  utils::{ match_pattern, Lookup },
};

use super::comment::CommentParser;

#[derive(Debug, Clone)]
pub struct BlockParser;

impl BlockParser {
  pub fn new(parser: &mut Parser) -> Result<Box<Node>, ParserError> {
    Ok(
      parser
        .get_statement(
          &mut LoopArgument::safe(
            "block_parser",
            &[],
            &[],
            &[
              (CommentParser::test, CommentParser::parse),
              (BlockParser::test, BlockParser::parse),
            ]
          )
        )?
        .unwrap()
    )
  }

  pub fn new_short(parser: &mut Parser, breakers: &[TokenType]) -> Result<Box<Node>, ParserError> {
    let start_loc = parser.tokens.get(parser.position).unwrap().get_location().unwrap();
    parser.position += 1;
    Ok(
      BlockNode::new(
        parser.get_children(
          &mut LoopArgument::with_tokens("block_short", &[TokenType::Semicolon], breakers)
        )?,
        parser.gen_loc(start_loc)
      )
    )
  }

  pub fn new_or_short(
    parser: &mut Parser,
    breakers: &[TokenType],
    args: &mut LoopArgument
  ) -> Result<(bool, Box<Node>), ParserError> {
    if let Some(start) = parser.tokens.get(parser.position) {
      return match start.token_type {
        TokenType::Colon => Ok((true, BlockParser::new_short(parser, breakers)?)),
        TokenType::LeftCurlyBracket => Ok((false, BlockParser::new(parser)?)),
        _ => Err(ParserError::internal("Block", args)),
      };
    }
    Err(ParserError::internal("Block", args))
  }

  pub fn new_or_short_or_single(
    parser: &mut Parser,
    breakers: &[TokenType],
    args: &mut LoopArgument
  ) -> Result<(bool, Box<Node>), ParserError> {
    if let Some(start) = parser.tokens.get(parser.position) {
      return match start.token_type {
        TokenType::Colon => Ok((true, BlockParser::new_short(parser, breakers)?)),
        TokenType::LeftCurlyBracket => Ok((false, BlockParser::new(parser)?)),
        _ => {
          let expr = guard!(
            parser.get_statement(
              &mut LoopArgument::safe("block_expr", &[], &[TokenType::Semicolon], &DEFAULT_PARSERS)
            )?,
            {
              return Err(ParserError::internal("Block", args));
            }
          );
          if let Some(token) = parser.tokens.get(parser.position) {
            if token.token_type == TokenType::Semicolon {
              parser.position += 1;
            }
          }
          Ok((false, expr))
        }
      };
    }
    Err(ParserError::internal("Block", args))
  }

  pub fn test(tokens: &[Token], _: &mut LoopArgument) -> Option<Vec<Vec<Token>>> {
    match_pattern(tokens, &[Lookup::Equal(&[TokenType::LeftCurlyBracket])])
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<Vec<Token>>,
    start_loc: Location,
    args: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [_] = matched.as_slice() {
      return Ok(
        BlockNode::new(
          parser.get_children(&mut LoopArgument::default("block_parser"))?,
          parser.gen_loc(start_loc)
        )
      );
    }
    Err(ParserError::internal("Exit", args))
  }
}
