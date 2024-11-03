use crate::{
  guard,
  lexer::token::{ Token, TokenType },
  parser::{
    node::Node,
    nodes::objectaccess::ObjectAccessNode,
    parser::{ Internal, LoopArgument, Parser },
    utils::{ match_pattern, Lookup },
  },
};

use super::identifier::IdentifierParser;

#[derive(Debug, Clone)]
pub struct ObjectAccessParser {}

impl Internal for ObjectAccessParser {
  fn test(&self, tokens: &Vec<Token>, _: &LoopArgument) -> Option<Vec<Vec<Token>>> {
    if
      let Some(m) = match_pattern(
        tokens,
        [
          Lookup::Equal(vec![TokenType::ObjectAccess]),
          Lookup::Equal(vec![TokenType::Identifier]),
        ].to_vec()
      )
    {
      return Some(m);
    }
    match_pattern(tokens, [Lookup::Equal(vec![TokenType::ObjectAccessBracketOpen])].to_vec())
  }

  fn parse(
    &self,
    parser: &mut Parser,
    matched: Vec<Vec<Token>>,
    args: &LoopArgument
  ) -> Option<Node> {
    match matched.len() {
      2 => {
        if let [_, prop] = matched.as_slice() {
          return Some(
            ObjectAccessNode::new(
              args.last_expr.to_owned().unwrap(),
              IdentifierParser::from_matched(prop)
            )
          );
        }
      }
      1 => {
        let expr = guard!(
          parser.get_statement(
            &mut LoopArgument::with_tokens(
              "objectaccess",
              &[TokenType::ObjectAccessBracketClose],
              &[]
            )
          )
        );
        parser.position += 1;
        return Some(ObjectAccessNode::new(args.last_expr.to_owned().unwrap(), expr));
      }
      _ => {
        return None;
      }
    }
    None
  }
}
