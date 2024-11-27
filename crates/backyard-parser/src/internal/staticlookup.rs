use backyard_lexer::token::{ Token, TokenType };
use backyard_nodes::node::{ ClassKeywordNode, Location, Node, StaticLookupNode };

use crate::{
  error::ParserError,
  parser::{ LocationHelper, LoopArgument, Parser },
  utils::{ match_pattern, Lookup },
};

use super::{ identifier::IdentifierParser, variable::VariableParser };

#[derive(Debug, Clone)]
pub struct StaticLookupParser;

impl StaticLookupParser {
  pub fn test(tokens: &[Token], args: &mut LoopArgument) -> Option<Vec<Vec<Token>>> {
    args.last_expr.as_ref()?;
    match_pattern(tokens, &[Lookup::Equal(&[TokenType::DoubleColon])])
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<Vec<Token>>,
    start_loc: Location,
    args: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [_] = matched.as_slice() {
      let left = args.last_expr.to_owned().unwrap();
      args.last_expr = None;
      if let Some(t) = parser.tokens.get(parser.position) {
        let expr = if t.token_type == TokenType::Class {
          parser.position += 1;
          ClassKeywordNode::new(parser.gen_loc_helper(t))
        } else if [TokenType::Variable, TokenType::VariableBracketOpen].contains(&t.token_type) {
          if let Some(m) = VariableParser::test(&parser.tokens[parser.position..], args) {
            parser.position += 1;
            VariableParser::parse(parser, m, t.get_location().unwrap(), args)?
          } else {
            return Err(ParserError::internal("StaticLookup 1", args));
          }
        } else if t.token_type == TokenType::LeftCurlyBracket {
          parser.position += 1;
          let expr = parser.get_statement(
            &mut LoopArgument::with_tokens("staticlookup", &[], &[TokenType::RightCurlyBracket])
          )?;
          parser.position += 1;
          if let Some(expr) = expr {
            return Ok(StaticLookupNode::new(left, expr, true, parser.gen_loc(start_loc)));
          } else {
            return Err(ParserError::internal("StaticLookup 2", args));
          }
        } else {
          parser.position += 1;
          IdentifierParser::from_token(t)
        };
        return Ok(StaticLookupNode::new(left, expr, false, parser.gen_loc(start_loc)));
      };
    }
    Err(ParserError::internal("StaticLookup 3", args))
  }
}
