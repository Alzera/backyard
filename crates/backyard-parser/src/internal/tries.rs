use backyard_lexer::token::{ Token, TokenType };
use backyard_nodes::node::{ CatchNode, Node, TryNode };
use utils::guard_none;

use crate::{ parser::{ LoopArgument, Parser }, utils::{ match_pattern, Lookup } };

use super::{
  block::BlockParser,
  comment::CommentParser,
  identifier::IdentifierParser,
  variable::VariableParser,
};

#[derive(Debug, Clone)]
pub struct TryParser {}

impl TryParser {
  pub fn test(tokens: &Vec<Token>, _: &mut LoopArgument) -> Option<Vec<Vec<Token>>> {
    match_pattern(tokens, [Lookup::Equal(vec![TokenType::Try])].to_vec())
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<Vec<Token>>,
    _: &mut LoopArgument
  ) -> Option<Box<Node>> {
    if let [_] = matched.as_slice() {
      let body = BlockParser::new(parser);
      let mut catches: Vec<Box<Node>> = vec![];
      let mut finally = None;
      loop {
        let is_finally = match guard_none!(parser.tokens.get(parser.position)).token_type {
          TokenType::Finally => true,
          TokenType::Catch => false,
          _ => {
            break;
          }
        };
        parser.position += 1;
        if is_finally {
          finally = Some(BlockParser::new(parser));
          break;
        }
        parser.position += 1;
        let types = parser.get_children(
          &mut LoopArgument::new(
            "catch_types",
            &[TokenType::BitwiseOr],
            &[TokenType::Variable, TokenType::VariableBracketOpen],
            &[
              (IdentifierParser::test, IdentifierParser::parse),
              (CommentParser::test, CommentParser::parse),
            ]
          )
        );
        parser.position -= 1;
        let variable = guard_none!(
          parser.get_statement(
            &mut LoopArgument::new(
              "catch_variable",
              &[],
              &[TokenType::RightParenthesis],
              &[
                (VariableParser::test, VariableParser::parse),
                (CommentParser::test, CommentParser::parse),
              ]
            )
          )
        );
        parser.position += 1;
        catches.push(CatchNode::new(types, variable, BlockParser::new(parser)));
      }
      return Some(TryNode::new(body, catches, finally));
    }
    None
  }
}
