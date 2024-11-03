use crate::{
  guard,
  lexer::token::{ Token, TokenType, TokenTypeArrayCombine },
  parser::{
    node::Node,
    nodes::yields::{ YieldFromNode, YieldNode },
    parser::{ Internal, LoopArgument, Parser },
    utils::{ match_pattern, Lookup },
  },
};

#[derive(Debug, Clone)]
pub struct YieldParser {}

impl Internal for YieldParser {
  fn test(&self, tokens: &Vec<Token>, _: &LoopArgument) -> Option<Vec<Vec<Token>>> {
    match_pattern(
      tokens,
      [Lookup::Equal(vec![TokenType::Yield]), Lookup::Optional(vec![TokenType::From])].to_vec()
    )
  }

  fn parse(
    &self,
    parser: &mut Parser,
    matched: Vec<Vec<Token>>,
    args: &LoopArgument
  ) -> Option<Node> {
    if let [_, has_from] = matched.as_slice() {
      let mut value = guard!(
        parser.get_statement(
          &mut LoopArgument::with_tokens(
            "singles",
            &args.separators.combine(&[]),
            &args.breakers.combine(&[TokenType::Arrow, TokenType::Semicolon])
          )
        )
      );
      if has_from.len() > 0 {
        return Some(YieldFromNode::new(value));
      }
      let mut key = None;
      if guard!(parser.tokens.get(parser.position)).token_type == TokenType::Arrow {
        key = Some(value);
        parser.position += 1;
        value = guard!(
          parser.get_statement(
            &mut LoopArgument::with_tokens(
              "singles",
              &args.separators.combine(&[]),
              &args.breakers.combine(&[TokenType::Semicolon])
            )
          )
        );
      }
      return Some(YieldNode::new(key, value));
    }
    None
  }
}
