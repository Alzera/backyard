use backyard_lexer::token::{ Token, TokenType };
use backyard_nodes::node::{ Location, Node, RangeLocation, VariableNode };

use crate::{
  error::ParserError,
  parser::{ LocationHelper, LoopArgument, Parser },
  utils::{ match_pattern, Lookup },
};

use super::identifier::IdentifierParser;

#[derive(Debug, Clone)]
pub struct VariableParser;

impl VariableParser {
  pub fn from_token(name: &Token) -> Box<Node> {
    let id = IdentifierParser::from_token(name);
    let loc = id.loc.clone();
    VariableParser::new_bracked(id, loc)
  }

  pub fn new_bracked(name: Box<Node>, loc: Option<RangeLocation>) -> Box<Node> {
    VariableNode::new(name, loc)
  }
}

impl VariableParser {
  pub fn test(tokens: &[Token], _: &mut LoopArgument) -> Option<Vec<Vec<Token>>> {
    match_pattern(tokens, &[Lookup::Equal(&[TokenType::Variable, TokenType::VariableBracketOpen])])
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<Vec<Token>>,
    start_loc: Location,
    args: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [name] = matched.as_slice() {
      if let Some(name) = name.first() {
        if name.token_type == TokenType::VariableBracketOpen {
          let expr = parser.get_statement(
            &mut LoopArgument::with_tokens("variable", &[], &[TokenType::VariableBracketClose])
          )?;
          parser.position += 1;
          if let Some(expr) = expr {
            let end_loc = parser.tokens.get(parser.position).unwrap().get_location().unwrap();
            return Ok(
              VariableParser::new_bracked(
                expr,
                Some(RangeLocation { start: start_loc, end: end_loc })
              )
            );
          }
        } else {
          return Ok(VariableParser::from_token(name));
        }
      }
    }
    Err(ParserError::internal("Variable", args))
  }
}
