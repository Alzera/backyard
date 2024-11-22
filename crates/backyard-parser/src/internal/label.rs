use backyard_lexer::token::{ Token, TokenType };
use backyard_nodes::node::{ LabelNode, Node, NodeType };
use utils::guard;

use crate::{
  error::ParserError,
  parser::{ LoopArgument, Parser },
  utils::{ match_pattern, Lookup },
};

#[derive(Debug, Clone)]
pub struct LabelParser {}

impl LabelParser {
  pub fn test(tokens: &Vec<Token>, args: &mut LoopArgument) -> Option<Vec<Vec<Token>>> {
    if let Some(last) = &args.last_expr {
      if last.node_type != NodeType::Identifier {
        return None;
      }
    }
    match_pattern(tokens, [Lookup::Equal(vec![TokenType::Colon])].to_vec())
  }

  pub fn parse(
    _: &mut Parser,
    matched: Vec<Vec<Token>>,
    args: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [_] = matched.as_slice() {
      let name = guard!(args.last_expr.to_owned(), {
        return Err(ParserError::internal("Label", args));
      });
      return Ok(LabelNode::new(name));
    }
    Err(ParserError::internal("Label", args))
  }
}
