use backyard_lexer::token::{ Token, TokenType };
use backyard_nodes::node::{ CatchNode, Node, TryNode };
use utils::guard;

use crate::{
  error::ParserError,
  parser::{ LoopArgument, Parser },
  utils::{ match_pattern, Lookup },
};

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
    args: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [_] = matched.as_slice() {
      let body = BlockParser::new(parser)?;
      let mut catches: Vec<Box<Node>> = vec![];
      let mut finally = None;
      loop {
        let is_finally = match
          guard!(parser.tokens.get(parser.position), {
            break;
          }).token_type
        {
          TokenType::Finally => true,
          TokenType::Catch => false,
          _ => {
            break;
          }
        };
        parser.position += 1;
        if is_finally {
          finally = Some(BlockParser::new(parser)?);
          break;
        }
        parser.position += 1;
        let types = parser.get_children(
          &mut LoopArgument::new(
            "catch_types",
            &[TokenType::BitwiseOr],
            &[TokenType::Variable, TokenType::VariableBracketOpen, TokenType::RightParenthesis],
            &[
              (IdentifierParser::test, IdentifierParser::parse),
              (CommentParser::test, CommentParser::parse),
            ]
          )
        )?;
        parser.position -= 1;
        let mut variable = None;
        if let Some(last_token) = parser.tokens.get(parser.position) {
          if last_token.token_type != TokenType::RightParenthesis {
            variable = parser.get_statement(
              &mut LoopArgument::new(
                "catch_variable",
                &[],
                &[TokenType::RightParenthesis],
                &[
                  (VariableParser::test, VariableParser::parse),
                  (CommentParser::test, CommentParser::parse),
                ]
              )
            )?;
          }
        }
        parser.position += 1;
        catches.push(CatchNode::new(types, variable, BlockParser::new(parser)?));
      }
      return Ok(TryNode::new(body, catches, finally));
    }
    Err(ParserError::internal("Try", args))
  }
}
