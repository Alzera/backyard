use crate::{
  lexer::token::{ Token, TokenType },
  parser::{
    node::Node,
    nodes::variable::VariableNode,
    parser::{ LoopArgument, Parser },
    utils::{ match_pattern, Lookup },
  },
};

use super::identifier::IdentifierParser;

#[derive(Debug, Clone)]
pub struct VariableParser {}

impl VariableParser {
  pub fn new(name: String, is_ref: bool) -> Node {
    VariableParser::new_bracked(IdentifierParser::new(name), is_ref)
  }

  pub fn new_bracked(name: Node, is_ref: bool) -> Node {
    VariableNode::new(is_ref, name)
  }
}

impl VariableParser {
  pub fn test(tokens: &Vec<Token>, _: &mut LoopArgument) -> Option<Vec<Vec<Token>>> {
    match_pattern(
      tokens,
      [
        Lookup::Optional(vec![TokenType::Reference]),
        Lookup::Equal(vec![TokenType::Variable, TokenType::VariableBracketOpen]),
      ].to_vec()
    )
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<Vec<Token>>,
    _: &mut LoopArgument
  ) -> Option<Node> {
    if matched.len() == 2 {
      if let [is_ref, name] = matched.as_slice() {
        if let Some(name) = name.get(0) {
          if name.token_type == TokenType::VariableBracketOpen {
            let expr = parser.get_statement(
              &mut LoopArgument::with_tokens("variable", &[TokenType::VariableBracketClose], &[])
            );
            parser.position += 1;
            if expr.is_some() {
              return Some(VariableParser::new_bracked(expr.unwrap(), is_ref.len() > 0));
            }
          } else {
            return Some(VariableParser::new(name.value.to_owned(), is_ref.len() > 0));
          }
        }
      }
    }
    None
  }
}
