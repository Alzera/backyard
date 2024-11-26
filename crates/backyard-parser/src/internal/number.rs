use backyard_lexer::token::{ Token, TokenType };
use backyard_nodes::node::{ Node, NumberNode };

use crate::{ error::ParserError, parser::{ LoopArgument, Parser }, utils::some_or_default };

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
    _: &mut Parser,
    matched: Vec<Vec<Token>>,
    args: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [number] = matched.as_slice() {
      return Ok(
        NumberNode::new(some_or_default(number.first(), String::from("0"), |i| i.value.to_owned()))
      );
    }
    Err(ParserError::internal("Number", args))
  }
}
