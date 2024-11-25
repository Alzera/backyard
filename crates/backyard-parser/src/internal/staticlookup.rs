use backyard_lexer::token::{ Token, TokenType };
use backyard_nodes::node::{ ClassKeywordNode, Node, StaticLookupNode };

use crate::{
  error::ParserError,
  parser::{ LoopArgument, Parser },
  utils::{ match_pattern, Lookup },
};

use super::{ identifier::IdentifierParser, variable::VariableParser };

#[derive(Debug, Clone)]
pub struct StaticLookupParser {}

impl StaticLookupParser {
  pub fn test(tokens: &Vec<Token>, args: &mut LoopArgument) -> Option<Vec<Vec<Token>>> {
    if args.last_expr.is_none() {
      return None;
    }
    match_pattern(tokens, [Lookup::Equal(vec![TokenType::DoubleColon])].to_vec())
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<Vec<Token>>,
    args: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [_] = matched.as_slice() {
      let left = args.last_expr.to_owned().unwrap();
      args.last_expr = None;
      if let Some(t) = parser.tokens.get(parser.position) {
        let expr = if t.token_type == TokenType::Class {
          parser.position += 1;
          ClassKeywordNode::new()
        } else if [TokenType::Variable, TokenType::VariableBracketOpen].contains(&t.token_type) {
          if let Some(m) = VariableParser::test(&parser.tokens[parser.position..].to_vec(), args) {
            parser.position += 1;
            VariableParser::parse(parser, m, args)?
          } else {
            return Err(ParserError::internal("StaticLookup 1", args));
          }
        } else {
          parser.position += 1;
          IdentifierParser::new(t.value.to_owned())
        };
        return Ok(StaticLookupNode::new(left, expr));
      };
    }
    Err(ParserError::internal("StaticLookup 3", args))
  }
}
