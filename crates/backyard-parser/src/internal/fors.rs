use backyard_lexer::token::{ Token, TokenType };
use backyard_nodes::node::{ BodyType, Node, ForNode };

use crate::{ parser::{ LoopArgument, Parser }, utils::{ match_pattern, Lookup } };

use super::block::BlockParser;

#[derive(Debug, Clone)]
pub struct ForParser {}

impl ForParser {
  pub fn test(tokens: &Vec<Token>, _: &mut LoopArgument) -> Option<Vec<Vec<Token>>> {
    match_pattern(
      tokens,
      [
        Lookup::Equal(vec![TokenType::For]),
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
      let inits = parser.get_children(
        &mut LoopArgument::with_tokens("for_inits", &[TokenType::Comma], &[TokenType::Semicolon])
      );
      let tests = parser.get_children(
        &mut LoopArgument::with_tokens("for_tests", &[TokenType::Comma], &[TokenType::Semicolon])
      );
      let increments = parser.get_children(
        &mut LoopArgument::with_tokens(
          "for_increments",
          &[TokenType::Comma],
          &[TokenType::RightParenthesis]
        )
      );
      let parsed_block = BlockParser::new_or_short(parser, &[TokenType::EndFor]);
      let mut body = None;
      let mut body_type = BodyType::Empty;
      if parsed_block.is_some() {
        let (is_short, parsed_block) = parsed_block.unwrap();
        body_type = match is_short {
          true => BodyType::Short,
          false => BodyType::Basic,
        };
        body = Some(parsed_block);
      }
      return Some(ForNode::new(inits, tests, increments, body, body_type));
    }
    None
  }
}
