use backyard_lexer::token::{ Token, TokenType };
use backyard_nodes::node::{ Location, Node, NumberNode };

use crate::{ error::ParserError, parser::{ LoopArgument, Parser } };

#[derive(Debug, Clone)]
pub struct NumberParser;

impl NumberParser {
  pub fn test(tokens: &[Token], _: &mut LoopArgument) -> Option<Vec<Vec<Token>>> {
    let token = tokens.first().unwrap();
    if [TokenType::Number, TokenType::NumberHex].contains(&token.token_type) {
      return Some(vec![vec![token.to_owned()]]);
    }
    None
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<Vec<Token>>,
    start_loc: Location,
    args: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [number] = matched.as_slice() {
      return Ok(
        NumberNode::new(
          number
            .first().map(|i| i.value.to_owned())
            .unwrap_or("0".to_string()),
          parser.gen_loc(start_loc)
        )
      );
    }
    Err(ParserError::internal("Number", args))
  }
}
