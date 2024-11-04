use crate::{
  guard,
  lexer::token::{ Token, TokenType },
  parser::{
    node::Node,
    nodes::staticlookup::StaticLookupNode,
    parser::{ Internal, LoopArgument, Parser },
    utils::{ match_pattern, Lookup },
  },
};

use super::identifier::IdentifierParser;

#[derive(Debug, Clone)]
pub struct StaticLookupParser {}

impl Internal for StaticLookupParser {
  fn test(&self, tokens: &Vec<Token>, _: &LoopArgument) -> Option<Vec<Vec<Token>>> {
    match_pattern(
      tokens,
      [
        Lookup::Equal(vec![TokenType::DoubleColon]),
        Lookup::Equal(vec![TokenType::Identifier, TokenType::Class]),
      ].to_vec()
    )
  }

  fn parse(&self, _: &mut Parser, matched: Vec<Vec<Token>>, args: &LoopArgument) -> Option<Node> {
    if let [_, prop] = matched.as_slice() {
      let on = guard!(args.last_expr.to_owned());
      return Some(StaticLookupNode::new(on, IdentifierParser::from_matched(prop)));
    }
    None
  }
}
