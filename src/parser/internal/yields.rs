use crate::{
  guard_none,
  lexer::token::{ Token, TokenType, TokenTypeArrayCombine },
  parser::{
    node::Node,
    nodes::yields::{ YieldFromNode, YieldNode },
    parser::{ LoopArgument, Parser },
    utils::{ match_pattern, Lookup },
  },
};

#[derive(Debug, Clone)]
pub struct YieldParser {}

impl YieldParser {
  pub fn test(tokens: &Vec<Token>, _: &mut LoopArgument) -> Option<Vec<Vec<Token>>> {
    match_pattern(
      tokens,
      [Lookup::Equal(vec![TokenType::Yield]), Lookup::Optional(vec![TokenType::From])].to_vec()
    )
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<Vec<Token>>,
    args: &mut LoopArgument
  ) -> Option<Node> {
    if let [_, has_from] = matched.as_slice() {
      let mut value = guard_none!(
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
      if guard_none!(parser.tokens.get(parser.position)).token_type == TokenType::Arrow {
        key = Some(value);
        parser.position += 1;
        value = guard_none!(
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
