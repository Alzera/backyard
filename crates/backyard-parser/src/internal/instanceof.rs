use backyard_lexer::token::{ Token, TokenType, TokenTypeArrayCombine };
use backyard_nodes::node::{ Node, InstanceOfNode };
use utils::guard;

use crate::{
  error::ParserError,
  parser::{ LoopArgument, Parser },
  utils::{ match_pattern, Lookup },
};

use super::types::TypesParser;

#[derive(Debug, Clone)]
pub struct InstanceOfParser {}

impl InstanceOfParser {
  pub fn test(tokens: &Vec<Token>, _: &mut LoopArgument) -> Option<Vec<Vec<Token>>> {
    match_pattern(tokens, [Lookup::Equal(vec![TokenType::InstanceOf])].to_vec())
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<Vec<Token>>,
    args: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [_] = matched.as_slice() {
      let right = guard!(
        parser.get_statement(
          &mut LoopArgument::new(
            "instanceof",
            &args.separators.combine(&[TokenType::Semicolon]),
            &args.breakers,
            &[(TypesParser::test, TypesParser::parse)]
          )
        )?,
        {
          return Err(ParserError::internal("InstanceOf", args));
        }
      );
      return Ok(InstanceOfNode::new(args.last_expr.to_owned().unwrap(), right));
    }
    Err(ParserError::internal("InstanceOf", args))
  }
}
