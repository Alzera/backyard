use backyard_lexer::token::{ Token, TokenType };
use backyard_nodes::node::{ ClassKeywordNode, Node, StaticLookupNode };
use utils::guard;

use crate::{
  error::ParserError,
  parser::{ LoopArgument, Parser },
  utils::{ match_pattern, Lookup },
};

use super::{ call::CallParser, identifier::IdentifierParser, variable::VariableParser };

#[derive(Debug, Clone)]
pub struct StaticLookupParser {}

impl StaticLookupParser {
  pub fn test(tokens: &Vec<Token>, _: &mut LoopArgument) -> Option<Vec<Vec<Token>>> {
    match_pattern(tokens, [Lookup::Equal(vec![TokenType::DoubleColon])].to_vec())
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<Vec<Token>>,
    args: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [_] = matched.as_slice() {
      let on = guard!(args.last_expr.to_owned(), {
        return Err(ParserError::internal("StaticLookup", args));
      });
      let expr = if
        guard!(parser.tokens.get(parser.position), {
          return Err(ParserError::internal("StaticLookup", args));
        }).token_type == TokenType::Class
      {
        parser.position += 1;
        ClassKeywordNode::new()
      } else {
        guard!(
          parser.get_statement(
            &mut LoopArgument::safe(
              "staticlookup",
              &[],
              &[TokenType::Semicolon],
              &[
                (CallParser::class_test, CallParser::parse),
                (VariableParser::test, VariableParser::parse),
                (IdentifierParser::test, IdentifierParser::parse),
              ]
            )
          )?,
          {
            return Err(ParserError::internal("StaticLookup: fail to parse basic", args));
          }
        )
      };
      return Ok(StaticLookupNode::new(on, expr));
    }
    Err(ParserError::internal("StaticLookup", args))
  }
}
