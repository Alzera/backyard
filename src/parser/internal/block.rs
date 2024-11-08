use crate::{
  lexer::token::TokenType,
  parser::{ node::Node, nodes::block::BlockNode, parser::{ LoopArgument, Parser } },
};

#[derive(Debug, Clone)]
pub struct BlockParser {}

impl BlockParser {
  pub fn new(parser: &mut Parser) -> Node {
    parser.position += 1;
    BlockNode::new(parser.get_children(&mut LoopArgument::default("block")))
  }

  pub fn new_short(parser: &mut Parser, breakers: &[TokenType]) -> Node {
    parser.position += 1;
    BlockNode::new(
      parser.get_children(
        &mut LoopArgument::with_tokens("block_short", &[TokenType::Semicolon], breakers)
      )
    )
  }

  pub fn new_or_short(parser: &mut Parser, breakers: &[TokenType]) -> Option<(bool, Node)> {
    if let Some(start) = parser.tokens.get(parser.position) {
      return match start.token_type {
        TokenType::Colon => Some((true, BlockParser::new_short(parser, breakers))),
        TokenType::LeftCurlyBracket => Some((false, BlockParser::new(parser))),
        _ => None,
      };
    }
    None
  }
}
