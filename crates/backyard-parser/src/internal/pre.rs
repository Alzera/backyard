use backyard_lexer::token::{ Token, TokenType };
use backyard_nodes::node::{ NegateNode, Node, PreNode, SilentNode };
use utils::guard_none;

use crate::{ parser::{ LoopArgument, Parser }, utils::{ match_pattern, Lookup } };

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
            TokenType::AtSign
          ]
        ),
      ].to_vec()
    )
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<Vec<Token>>,
    args: &mut LoopArgument
  ) -> Option<Box<Node>> {
    if let [operator] = matched.as_slice() {
      let operator = guard_none!(operator.get(0));
      let argument = guard_none!(
        parser.get_statement(&mut LoopArgument::with_tokens("pre", args.separators, args.breakers))
      );
      return match operator.token_type {
        TokenType::PreIncrement | TokenType::PreDecrement =>
          Some(PreNode::new(argument, operator.value.to_owned())),
        TokenType::BooleanNegate => Some(NegateNode::new(argument)),
        TokenType::AtSign => Some(SilentNode::new(argument)),
        _ => None,
      };
    }
    None
  }
}
