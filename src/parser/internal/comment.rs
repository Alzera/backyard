use crate::{
  guard_none,
  lexer::token::{ Token, TokenType },
  parser::{
    node::Node,
    nodes::comment::{ CommentBlockNode, CommentDocNode, CommentLineNode },
    parser::{ LoopArgument, Parser },
    utils::{ match_pattern, Lookup },
  },
};

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
  ) -> Option<Node> {
    if let [comment] = matched.as_slice() {
      let comment = guard_none!(comment.get(0));
      let comment: Node = match comment.token_type {
        TokenType::CommentLine => CommentLineNode::boxed(comment.value.to_owned()),
        TokenType::CommentBlock => CommentBlockNode::boxed(comment.value.to_owned()),
        TokenType::CommentDoc => CommentDocNode::boxed(comment.value.to_owned()),
        _ => {
          return None;
        }
      };
      let expr = parser.get_statement(
        &mut LoopArgument::new("comment", args.separators, args.breakers, args.parsers)
      );
      if let Some(mut expr) = expr {
        expr.add_leading_comments(comment);
        return Some(expr);
      }
      if let Some(expr) = args.statements.last_mut() {
        expr.add_trailing_comments(comment);
        return None;
      }
      return Some(comment);
    }
    None
  }
}
