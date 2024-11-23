use backyard_lexer::token::{ Token, TokenType };
use backyard_nodes::node::{ Node, CommentBlockNode, CommentDocNode, CommentLineNode };
use utils::guard;

use crate::{
  error::ParserError,
  parser::{ LoopArgument, Parser },
  utils::{ match_pattern, Lookup },
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
  ) -> Result<Box<Node>, ParserError> {
    if let [comment] = matched.as_slice() {
      let comment = guard!(comment.get(0), {
        return Err(ParserError::internal("Comment: failed to get type", args));
      });
      let comment: Box<Node> = match comment.token_type {
        TokenType::CommentLine => CommentLineNode::new(comment.value.to_owned()),
        TokenType::CommentBlock => CommentBlockNode::new(comment.value.to_owned()),
        TokenType::CommentDoc => CommentDocNode::new(comment.value.to_owned()),
        _ => {
          return Err(ParserError::internal("Comment: failed creating node", args));
        }
      };
      let expr = parser.get_statement(
        &mut LoopArgument::new("comment", args.separators, args.breakers, args.parsers)
      )?;
      if let Some(mut expr) = expr {
        expr.leadings.insert(0, comment);
        return Ok(expr);
      }
      if let Some(mut expr) = args.statements.pop() {
        expr.trailings.push(comment);
        return Ok(expr);
      }
      return Ok(comment);
    }
    Err(ParserError::internal("Comment", args))
  }
}
