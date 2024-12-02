use backyard_lexer::token::{ Token, TokenType };
use backyard_nodes::node::{ Location, Node, CommentBlockNode, CommentDocNode, CommentLineNode };

use crate::{
  error::ParserError,
  guard,
  parser::{ LoopArgument, Parser },
  utils::{ match_pattern, Lookup },
};

#[derive(Debug, Clone)]
pub struct CommentParser;

impl CommentParser {
  pub fn test(tokens: &[Token], _: &mut LoopArgument) -> Option<Vec<Vec<Token>>> {
    match_pattern(
      tokens,
      &[Lookup::Equal(&[TokenType::CommentLine, TokenType::CommentBlock, TokenType::CommentDoc])]
    )
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<Vec<Token>>,
    start_loc: Location,
    args: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [comment] = matched.as_slice() {
      let comment = guard!(comment.first(), {
        return Err(ParserError::internal("Comment: failed to get type", args));
      });
      let comment: Box<Node> = match comment.token_type {
        TokenType::CommentLine =>
          CommentLineNode::new(comment.value.to_owned(), parser.gen_loc(start_loc)),
        TokenType::CommentBlock =>
          CommentBlockNode::new(comment.value.to_owned(), parser.gen_loc(start_loc)),
        TokenType::CommentDoc =>
          CommentDocNode::new(comment.value.to_owned(), parser.gen_loc(start_loc)),
        _ => {
          return Err(ParserError::internal("Comment: failed creating node", args));
        }
      };
      let expr = parser.get_statement(args)?;
      if let Some(mut expr) = expr {
        expr.leadings.insert(0, comment);
        return Ok(expr);
      }
      if let Some(mut expr) = args.last_expr.to_owned() {
        expr.trailings.push(comment);
        return Ok(expr);
      }
      if let Some(expr) = args.statements.pop() {
        return Ok(expr);
      }
      return Ok(comment);
    }
    Err(ParserError::internal("Comment", args))
  }
}
