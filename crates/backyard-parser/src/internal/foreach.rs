use backyard_lexer::token::{ Token, TokenType };
use backyard_nodes::node::{ Location, Node, ForeachNode };

use crate::{
  error::ParserError,
  guard,
  parser::{ LoopArgument, Parser },
  utils::{ match_pattern, Lookup, LookupResult },
};

use super::block::BlockParser;

#[derive(Debug, Clone)]
pub struct ForeachParser;

impl ForeachParser {
  fn get_key_value(parser: &mut Parser) -> Result<(Option<Box<Node>>, Box<Node>), ParserError> {
    let key_or_value = guard!(
      parser.get_statement(
        &mut LoopArgument::with_tokens(
          "foreach_key_or_value",
          &[],
          &[TokenType::Arrow, TokenType::RightParenthesis]
        )
      )?
    );
    let has_key = guard!(parser.tokens.get(parser.position));
    parser.position += 1;
    if has_key.token_type == TokenType::RightParenthesis {
      return Ok((None, key_or_value));
    } else if has_key.token_type == TokenType::Arrow {
      let value = guard!(
        parser.get_statement(
          &mut LoopArgument::with_tokens("foreach_value", &[], &[TokenType::RightParenthesis])
        )?
      );
      parser.position += 1;
      return Ok((Some(key_or_value), value));
    }
    Err(ParserError::Internal)
  }
}

impl ForeachParser {
  pub fn test(tokens: &[Token], _: &mut LoopArgument) -> Option<Vec<LookupResult>> {
    match_pattern(
      tokens,
      &[Lookup::Equal(&[TokenType::Foreach]), Lookup::Equal(&[TokenType::LeftParenthesis])]
    )
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<LookupResult>,
    start_loc: Location,
    args: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [_, _] = matched.as_slice() {
      let source = guard!(
        parser.get_statement(
          &mut LoopArgument::with_tokens("foreach_source", &[], &[TokenType::As])
        )?
      );
      parser.position += 1;
      let (key, value) = ForeachParser::get_key_value(parser)?;
      let (is_short, body) = BlockParser::new_or_short_or_single(
        parser,
        &[TokenType::EndForeach],
        args
      )?;
      return Ok(ForeachNode::loc(source, key, value, body, is_short, parser.gen_loc(start_loc)));
    }
    Err(ParserError::Internal)
  }
}
