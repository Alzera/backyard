use backyard_lexer::token::{ Token, TokenType };
use backyard_nodes::node::{ BodyType, Location, Node, ForNode };

use crate::{
  error::ParserError,
  parser::{ LoopArgument, Parser },
  utils::{ match_pattern, Lookup },
};

use super::block::BlockParser;

#[derive(Debug, Clone)]
pub struct ForParser;

impl ForParser {
  pub fn test(tokens: &[Token], _: &mut LoopArgument) -> Option<Vec<Vec<Token>>> {
    match_pattern(
      tokens,
      &[Lookup::Equal(&[TokenType::For]), Lookup::Equal(&[TokenType::LeftParenthesis])]
    )
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<Vec<Token>>,
    start_loc: Location,
    args: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [_, _] = matched.as_slice() {
      let inits = parser.get_children(
        &mut LoopArgument::with_tokens("for_inits", &[TokenType::Comma], &[TokenType::Semicolon])
      )?;
      let tests = parser.get_children(
        &mut LoopArgument::with_tokens("for_tests", &[TokenType::Comma], &[TokenType::Semicolon])
      )?;
      let increments = parser.get_children(
        &mut LoopArgument::with_tokens(
          "for_increments",
          &[TokenType::Comma],
          &[TokenType::RightParenthesis]
        )
      )?;
      let parsed_block = BlockParser::new_or_short(parser, &[TokenType::EndFor], args);
      let mut body = None;
      let mut body_type = BodyType::Empty;
      if parsed_block.is_ok() {
        let (is_short, parsed_block) = parsed_block.unwrap();
        body_type = match is_short {
          true => BodyType::Short,
          false => BodyType::Basic,
        };
        body = Some(parsed_block);
      }
      return Ok(ForNode::new(inits, tests, increments, body, body_type, parser.gen_loc(start_loc)));
    }
    Err(ParserError::internal("For", args))
  }
}
