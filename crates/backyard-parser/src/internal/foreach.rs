use backyard_lexer::token::{ Token, TokenType };
use backyard_nodes::node::{ Node, ForeachNode };
use utils::guard;

use crate::{
  error::ParserError,
  parser::{ LoopArgument, Parser },
  utils::{ match_pattern, Lookup },
};

use super::block::BlockParser;

#[derive(Debug, Clone)]
pub struct ForeachParser;

impl ForeachParser {
  fn get_key_value(
    parser: &mut Parser,
    args: &LoopArgument
  ) -> Result<(Option<Box<Node>>, Box<Node>), ParserError> {
    let key_or_value = guard!(
      parser.get_statement(
        &mut LoopArgument::with_tokens(
          "foreach_key_or_value",
          &[],
          &[TokenType::Arrow, TokenType::RightParenthesis]
        )
      )?,
      {
        return Err(ParserError::internal("Foreach", args));
      }
    );
    let has_key = guard!(parser.tokens.get(parser.position), {
      return Err(ParserError::internal("Foreach", args));
    });
    parser.position += 1;
    if has_key.token_type == TokenType::RightParenthesis {
      return Ok((None, key_or_value));
    } else if has_key.token_type == TokenType::Arrow {
      let value = guard!(
        parser.get_statement(
          &mut LoopArgument::with_tokens("foreach_value", &[], &[TokenType::RightParenthesis])
        )?,
        {
          return Err(ParserError::internal("Foreach", args));
        }
      );
      parser.position += 1;
      return Ok((Some(key_or_value), value));
    }
    Err(ParserError::internal("Foreach", args))
  }
}

impl ForeachParser {
  pub fn test(tokens: &[Token], _: &mut LoopArgument) -> Option<Vec<Vec<Token>>> {
    match_pattern(
      tokens,
      [
        Lookup::Equal(vec![TokenType::Foreach]),
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
      let source = guard!(
        parser.get_statement(
          &mut LoopArgument::with_tokens("foreach_source", &[], &[TokenType::As])
        )?,
        {
          return Err(ParserError::internal("Foreach", args));
        }
      );
      parser.position += 1;
      let (key, value) = ForeachParser::get_key_value(parser, args)?;
      let (is_short, body) = BlockParser::new_or_short(parser, &[TokenType::EndForeach], args)?;
      return Ok(ForeachNode::new(source, key, value, body, is_short));
    }
    Err(ParserError::internal("Foreach", args))
  }
}
