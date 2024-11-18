use backyard_lexer::token::{ Token, TokenType };
use backyard_nodes::node::{ Node, CommentBlockNode, CommentDocNode, CommentLineNode };
use utils::guard_none;

use crate::{ parser::{ LoopArgument, Parser }, utils::{ match_pattern, Lookup } };

#[derive(Debug, Clone)]
pub struct CommentParser {}

impl CommentParser {
  pub fn test(tokens: &Vec<Token>, _: &mut LoopArgument) -> Option<Vec<Vec<Token>>> {
    match_pattern(
      tokens,
      [
        Lookup::Equal(vec![TokenType::CommentLine, TokenType::CommentBlock, TokenType::CommentDoc]),
      ].to_vec()
    )
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<Vec<Token>>,
    args: &mut LoopArgument
  ) -> Option<Box<Node>> {
    if let [comment] = matched.as_slice() {
      let comment = guard_none!(comment.get(0));
      let comment: Box<Node> = match comment.token_type {
        TokenType::CommentLine => CommentLineNode::new(comment.value.to_owned()),
        TokenType::CommentBlock => CommentBlockNode::new(comment.value.to_owned()),
        TokenType::CommentDoc => CommentDocNode::new(comment.value.to_owned()),
        _ => {
          return None;
        }
      };
      let expr = parser.get_statement(
        &mut LoopArgument::new("comment", args.separators, args.breakers, args.parsers)
      );
      if let Some(mut expr) = expr {
        expr.leading_comments.push(comment);
        return Some(expr);
      }
      if let Some(expr) = args.statements.last_mut() {
        expr.trailing_comments.push(comment);
        return None;
      }
      return Some(comment);
    }
    None
  }
}
