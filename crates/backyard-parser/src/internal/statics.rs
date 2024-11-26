use backyard_lexer::token::{ Token, TokenType };
use backyard_nodes::node::{ Node, NodeType, StaticNode };

use crate::{ error::ParserError, parser::{ LoopArgument, Parser } };

use super::{ comment::CommentParser, property::PropertyItemParser };

#[derive(Debug, Clone)]
pub struct StaticsParser;

impl StaticsParser {
  pub fn test(tokens: &[Token], args: &mut LoopArgument) -> Option<Vec<Vec<Token>>> {
    if let Some(last_expr) = &args.last_expr {
      if last_expr.node_type == NodeType::StaticKeyword {
        if let Some(token) = tokens.first() {
          if [TokenType::Variable, TokenType::VariableBracketOpen].contains(&token.token_type) {
            return Some(vec![vec![]]);
          }
        }
      }
    }
    None
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<Vec<Token>>,
    args: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [_] = matched.as_slice() {
      let items = parser.get_children(
        &mut LoopArgument::new(
          "static",
          &[TokenType::Comma],
          &[TokenType::Semicolon],
          &[
            (CommentParser::test, CommentParser::parse),
            (PropertyItemParser::test, PropertyItemParser::parse),
          ]
        )
      )?;
      return Ok(StaticNode::new(items));
    }
    Err(ParserError::internal("StaticLookup", args))
  }
}
