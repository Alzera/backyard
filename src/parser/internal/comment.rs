use crate::{
  guard,
  lexer::token::{ Token, TokenType },
  parser::{
    node::Node,
    nodes::comment::{ CommentBlockNode, CommentLineNode },
    parser::{ LoopArgument, Parser },
    utils::{ match_pattern, Lookup },
  },
};

#[derive(Debug, Clone)]
pub struct CommentParser {}

impl CommentParser {
  pub fn test(tokens: &Vec<Token>, _: &LoopArgument) -> Option<Vec<Vec<Token>>> {
    match_pattern(
      tokens,
      [
        Lookup::Equal(vec![TokenType::CommentLine, TokenType::CommentBlock, TokenType::CommentDoc]),
      ].to_vec()
    )
  }

  pub fn parse(parser: &mut Parser, matched: Vec<Vec<Token>>, args: &LoopArgument) -> Option<Node> {
    if let [comment] = matched.as_slice() {
      let comment = guard!(comment.get(0));
      let comment: Node = if comment.token_type == TokenType::CommentLine {
        CommentLineNode::new(comment.value.to_owned())
      } else {
        CommentBlockNode::new(comment.value.to_owned())
      };
      let expr = parser.get_statement(
        &mut LoopArgument::new("comment", args.separators, args.breakers, args.parsers)
      );
      if let Some(mut expr) = expr {
        expr.add_leading_comments(comment);
        return Some(expr);
      }
      if let Some(mut expr) = args.last_expr.to_owned() {
        expr.add_trailing_comments(comment);
        return Some(expr);
      }
      return Some(comment);
    }
    None
  }
}
