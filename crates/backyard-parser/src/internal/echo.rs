use backyard_lexer::token::{ Token, TokenType, TokenTypeArrayCombine };
use backyard_nodes::node::{ EchoNode, Node };

use crate::{
  error::ParserError,
  parser::{ LoopArgument, Parser },
  utils::{ match_pattern, Lookup },
};

#[derive(Debug, Clone)]
pub struct EchoParser {}

impl EchoParser {
  pub fn test(tokens: &Vec<Token>, _: &mut LoopArgument) -> Option<Vec<Vec<Token>>> {
    match_pattern(tokens, [Lookup::Equal(vec![TokenType::Echo])].to_vec())
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<Vec<Token>>,
    args: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [_] = matched.as_slice() {
      let items = parser.get_children(
        &mut LoopArgument::with_tokens(
          "static",
          &[TokenType::Comma],
          &args.breakers.combine(&args.separators).combine(&[TokenType::Semicolon])
        )
      )?;
      if let Some(last_token) = parser.tokens.get(parser.position - 1) {
        if last_token.token_type == TokenType::Semicolon {
          parser.position -= 1;
        }
      }
      return Ok(EchoNode::new(items));
    }
    Err(ParserError::internal("StaticLookup", args))
  }
}
