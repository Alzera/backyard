use backyard_lexer::token::{ Token, TokenType };
use backyard_nodes::node::{ Node, NodeType, ArrayItemNode, ArrayNode };
use utils::guard;

use crate::{
  error::ParserError,
  parser::{ LoopArgument, Parser, DEFAULT_PARSERS },
  utils::{ match_pattern, Lookup },
};

#[derive(Debug, Clone)]
pub struct ArrayParser {}

impl ArrayParser {
  pub fn test(tokens: &Vec<Token>, _: &mut LoopArgument) -> Option<Vec<Vec<Token>>> {
    match_pattern(tokens, [Lookup::Equal(vec![TokenType::LeftSquareBracket])].to_vec())
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<Vec<Token>>,
    args: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [_] = matched.as_slice() {
      let mut loop_parsers = DEFAULT_PARSERS.to_vec();
      loop_parsers.insert(0, (ArrayItemParser::test, ArrayItemParser::parse));
      let values = parser
        .get_children(
          &mut LoopArgument::new(
            "array",
            &[TokenType::Comma],
            &[TokenType::RightSquareBracket],
            &loop_parsers
          )
        )?
        .iter()
        .map(|i| (
          if i.node_type == NodeType::ArrayItem {
            i.to_owned()
          } else {
            ArrayItemNode::new(None, i.to_owned())
          }
        ))
        .collect::<Vec<Box<Node>>>();
      return Ok(ArrayNode::new(values));
    }
    Err(ParserError::internal("Array", args))
  }
}

#[derive(Debug, Clone)]
pub struct ArrayItemParser {}

impl ArrayItemParser {
  pub fn test(tokens: &Vec<Token>, _: &mut LoopArgument) -> Option<Vec<Vec<Token>>> {
    match_pattern(tokens, [Lookup::Equal(vec![TokenType::Arrow])].to_vec())
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<Vec<Token>>,
    args: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [_] = matched.as_slice() {
      let value = guard!(
        parser.get_statement(
          &mut LoopArgument::with_tokens(
            "array_item",
            &[],
            &[TokenType::Comma, TokenType::RightSquareBracket]
          )
        )?,
        {
          return Err(ParserError::internal("ArrayItem", args));
        }
      );
      let key = args.last_expr.to_owned();
      return Ok(ArrayItemNode::new(key, value));
    }
    Err(ParserError::internal("ArrayItem", args))
  }
}
