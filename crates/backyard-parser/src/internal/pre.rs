use backyard_lexer::token::{ Token, TokenType };
use backyard_nodes::node::{ NegateNode, Node, PreNode, ReferenceNode, SilentNode, VariadicNode };

use crate::{
  error::ParserError,
  guard,
  parser::{ LoopArgument, Parser },
  utils::{ match_pattern, Lookup },
};

#[derive(Debug, Clone)]
pub struct PreParser;

impl PreParser {
  pub fn test(tokens: &[Token], _: &mut LoopArgument) -> Option<Vec<Vec<Token>>> {
    match_pattern(
      tokens,
      &[
        Lookup::Equal(
          &[
            TokenType::PreIncrement,
            TokenType::PreDecrement,
            TokenType::BooleanNegate,
            TokenType::AtSign,
            TokenType::Addition,
            TokenType::Subtraction,
            TokenType::Ellipsis,
            TokenType::BitwiseAnd,
          ]
        ),
      ]
    )
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<Vec<Token>>,
    args: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [operator] = matched.as_slice() {
      let operator = guard!(operator.first(), {
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
        | TokenType::PreIncrement
        | TokenType::PreDecrement
        | TokenType::Addition
        | TokenType::Subtraction => Ok(PreNode::new(argument, operator.value.to_owned())),
        TokenType::BooleanNegate => Ok(NegateNode::new(argument)),
        TokenType::AtSign => Ok(SilentNode::new(argument)),
        TokenType::BitwiseAnd => Ok(ReferenceNode::new(argument)),
        _ => Err(ParserError::internal("Pre", args)),
      };
    }
    Err(ParserError::internal("Pre", args))
  }
}
