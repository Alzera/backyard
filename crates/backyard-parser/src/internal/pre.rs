use backyard_lexer::token::{ Token, TokenType };
use backyard_nodes::node::{ NegateNode, Node, PreNode, SilentNode, VariadicNode };
use utils::guard;

use crate::{
  error::ParserError,
  parser::{ LoopArgument, Parser },
  utils::{ match_pattern, Lookup },
};

#[derive(Debug, Clone)]
pub struct PreParser {}

impl PreParser {
  pub fn test(tokens: &Vec<Token>, _: &mut LoopArgument) -> Option<Vec<Vec<Token>>> {
    match_pattern(
      tokens,
      [
        Lookup::Equal(
          vec![
            TokenType::PreIncrement,
            TokenType::PreDecrement,
            TokenType::BooleanNegate,
            TokenType::AtSign,
            TokenType::Subtraction,
            TokenType::Ellipsis
          ]
        ),
      ].to_vec()
    )
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<Vec<Token>>,
    args: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [operator] = matched.as_slice() {
      let operator = guard!(operator.get(0), {
        return Err(ParserError::internal("Pre", args));
      });
      let argument = parser.get_statement(
        &mut LoopArgument::with_tokens("pre", args.separators, args.breakers)
      )?;
      if operator.token_type == TokenType::Ellipsis {
        return Ok(VariadicNode::new(argument));
      }
      let argument = guard!(argument, {
        return Err(ParserError::internal("Pre", args));
      });
      return match operator.token_type {
        TokenType::PreIncrement | TokenType::PreDecrement =>
          Ok(PreNode::new(argument, operator.value.to_owned())),
        TokenType::BooleanNegate => Ok(NegateNode::new(argument)),
        TokenType::AtSign => Ok(SilentNode::new(argument)),
        TokenType::Subtraction => Ok(SilentNode::new(argument)),
        _ => Err(ParserError::internal("Pre", args)),
      };
    }
    Err(ParserError::internal("Pre", args))
  }
}
