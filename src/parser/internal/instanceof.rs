use crate::{
  guard,
  lexer::token::{ Token, TokenType, TokenTypeArrayCombine },
  parser::{
    node::Node,
    nodes::instanceof::InstanceOfNode,
    parser::{ Internal, LoopArgument, Parser },
    utils::{ match_pattern, Lookup },
  },
};

#[derive(Debug, Clone)]
pub struct InstanceOfParser {}

impl Internal for InstanceOfParser {
  fn test(&self, tokens: &Vec<Token>, _: &LoopArgument) -> Option<Vec<Vec<Token>>> {
    match_pattern(tokens, [Lookup::Equal(vec![TokenType::InstanceOf])].to_vec())
  }

  fn parse(
    &self,
    parser: &mut Parser,
    matched: Vec<Vec<Token>>,
    args: &LoopArgument
  ) -> Option<Node> {
    if let [_] = matched.as_slice() {
      let right = guard!(
        parser.get_statement(
          &mut LoopArgument::with_tokens(
            "instanceof",
            &args.separators.combine(&[TokenType::Semicolon]),
            &args.breakers
          )
        )
      );
      return Some(InstanceOfNode::new(args.last_expr.to_owned().unwrap(), right));
    }
    None
  }
}
