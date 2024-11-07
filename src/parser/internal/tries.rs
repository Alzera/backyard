use crate::{
  guard,
  lexer::token::{ Token, TokenType },
  parser::{
    node::Node,
    nodes::tries::{ CatchNode, TryNode },
    parser::{ LoopArgument, Parser },
    utils::{ match_pattern, Lookup },
  },
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
  pub fn test(tokens: &Vec<Token>, _: &LoopArgument) -> Option<Vec<Vec<Token>>> {
    match_pattern(
      tokens,
      [
        Lookup::Equal(vec![TokenType::Try]),
        Lookup::Equal(vec![TokenType::LeftCurlyBracket]),
      ].to_vec()
    )
  }

  pub fn parse(parser: &mut Parser, matched: Vec<Vec<Token>>, _: &LoopArgument) -> Option<Node> {
    if let [_, _] = matched.as_slice() {
      let body = BlockParser::new(parser);
      let mut catches: Vec<Node> = vec![];
      let mut finally = None;
      loop {
        let is_finally = match guard!(parser.tokens.get(parser.position)).token_type {
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
        let variable = guard!(
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
