use backyard_lexer::token::TokenType;
use backyard_nodes::node::{ BlockNode, Node };

use crate::{ error::ParserError, parser::{ LocationHelper, LoopArgument, Parser } };

#[derive(Debug, Clone)]
pub struct BlockParser;

impl BlockParser {
  pub fn new(parser: &mut Parser) -> Result<Box<Node>, ParserError> {
    let start_loc = parser.tokens.get(parser.position).unwrap().get_location().unwrap();
    parser.position += 1;
    Ok(
      BlockNode::new(
        parser.get_children(&mut LoopArgument::default("block"))?,
        parser.gen_loc(start_loc)
      )
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
}
