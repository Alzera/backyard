use backyard_lexer::token::{ Token, TokenType };
use backyard_nodes::node::{ Node, WhileNode };
use utils::guard;

use crate::{
  error::ParserError,
  parser::{ LoopArgument, Parser },
  utils::{ match_pattern, Lookup },
};

use super::block::BlockParser;

#[derive(Debug, Clone)]
pub struct WhileParser;

impl WhileParser {
  pub fn test(tokens: &[Token], _: &mut LoopArgument) -> Option<Vec<Vec<Token>>> {
    match_pattern(
      tokens,
      [
        Lookup::Equal(vec![TokenType::While]),
        Lookup::Equal(vec![TokenType::LeftParenthesis]),
      ].to_vec()
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
          &mut LoopArgument::with_tokens("while", &[], &[TokenType::RightParenthesis])
        )?,
        {
          return Err(ParserError::internal("While", args));
        }
      );
      parser.position += 1;
      let (is_short, body) = BlockParser::new_or_short(parser, &[TokenType::EndWhile], args)?;
      return Ok(WhileNode::new(condition, body, is_short));
    }
    Err(ParserError::internal("While", args))
  }
}
