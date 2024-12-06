use backyard_lexer::token::{ Token, TokenType };
use backyard_nodes::node::{ Location, Node, WhileNode };

use crate::{
  error::ParserError,
  guard,
  parser::{ LoopArgument, Parser },
  utils::{ match_pattern, Lookup, LookupResult },
};

use super::block::BlockParser;

#[derive(Debug, Clone)]
pub struct WhileParser;

impl WhileParser {
  pub fn test(tokens: &[Token], _: &mut LoopArgument) -> Option<Vec<LookupResult>> {
    match_pattern(
      tokens,
      &[Lookup::Equal(&[TokenType::While]), Lookup::Equal(&[TokenType::LeftParenthesis])]
    )
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<LookupResult>,
    start_loc: Location,
    args: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [_, _] = matched.as_slice() {
      let condition = guard!(
        parser.get_statement(
          &mut LoopArgument::with_tokens("while", &[], &[TokenType::RightParenthesis])
        )?
      );
      parser.position += 1;
      let (is_short, body) = BlockParser::new_or_short_or_single(
        parser,
        &[TokenType::EndWhile],
        args
      )?;
      return Ok(WhileNode::loc(condition, body, is_short, parser.gen_loc(start_loc)));
    }
    Err(ParserError::Internal)
  }
}
