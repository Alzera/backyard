use backyard_lexer::token::{ Token, TokenType };
use backyard_nodes::node::{ Node, ProgramNode };

use crate::{
  error::ParserError,
  parser::{ LoopArgument, Parser },
  utils::{ match_pattern, Lookup },
};

#[derive(Debug, Clone)]
pub struct ProgramParser {}

impl ProgramParser {
  pub fn test(tokens: &Vec<Token>, _: &mut LoopArgument) -> Option<Vec<Vec<Token>>> {
    match_pattern(
      tokens,
      [Lookup::Equal(vec![TokenType::OpenTag, TokenType::OpenTagShort])].to_vec()
    )
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<Vec<Token>>,
    args: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [_] = matched.as_slice() {
      let program = parser.get_children(&mut LoopArgument::default("main"))?;
      return Ok(ProgramNode::new(program));
    }
    Err(ParserError::internal("Program", args))
  }
}
