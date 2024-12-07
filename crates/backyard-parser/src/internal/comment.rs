use backyard_lexer::token::{ Token, TokenType };
use backyard_nodes::node::{
  CommentBlockNode,
  CommentDocNode,
  CommentLineNode,
  Location,
  Node,
  NodeType,
};

use crate::{
  error::ParserError,
  parser::{ LoopArgument, Parser },
  utils::{ match_pattern, Lookup, LookupResult },
};

#[derive(Debug, Clone)]
pub struct CommentParser;

impl CommentParser {
  pub fn test(tokens: &[Token], _: &mut LoopArgument) -> Option<Vec<LookupResult>> {
    match_pattern(
      tokens,
      &[Lookup::Equal(&[TokenType::CommentLine, TokenType::CommentBlock, TokenType::CommentDoc])]
    )
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<LookupResult>,
    start_loc: Location,
    args: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [comment] = matched.as_slice() {
      let comment = comment.as_equal()?;
      let comment: Box<Node> = match comment.token_type {
        TokenType::CommentLine =>
          CommentLineNode::loc(comment.value.to_owned(), parser.gen_loc(start_loc)),
        TokenType::CommentBlock =>
          CommentBlockNode::loc(comment.value.to_owned(), parser.gen_loc(start_loc)),
        TokenType::CommentDoc =>
          CommentDocNode::loc(comment.value.to_owned(), parser.gen_loc(start_loc)),
        _ => {
          return Err(ParserError::Internal);
        }
      };
      let expr = parser.get_statement(
        &mut LoopArgument::safe("comment", args.separators, args.breakers, args.parsers)
      )?;
      if let Some(expr) = &expr {
        if
          ![NodeType::CommentBlock, NodeType::CommentDoc, NodeType::CommentLine].contains(
            &expr.node_type
          )
        {
          let mut expr = expr.to_owned();
          if let Some(expr_loc) = &expr.loc {
            if let Some(comment_loc) = &comment.loc {
              if expr_loc.start.offset < comment_loc.start.offset {
                expr.trailings.push(comment);
              } else {
                expr.leadings.insert(0, comment);
              }
            }
          }
          return Ok(expr);
        }
      }
      if let Some(mut last_expr) = args.last_expr.to_owned() {
        if
          ![NodeType::CommentBlock, NodeType::CommentDoc, NodeType::CommentLine].contains(
            &last_expr.node_type
          )
        {
          last_expr.trailings.push(comment);
          if let Some(next_expr) = expr {
            if
              [NodeType::CommentBlock, NodeType::CommentDoc, NodeType::CommentLine].contains(
                &next_expr.node_type
              )
            {
              last_expr.trailings.push(next_expr);
            }
          }
          return Ok(last_expr);
        }
      }
      if let Some(expr) = args.statements.pop() {
        if
          ![NodeType::CommentBlock, NodeType::CommentDoc, NodeType::CommentLine].contains(
            &expr.node_type
          )
        {
          return Ok(expr);
        }
      }
      return Ok(comment);
    }
    Err(ParserError::Internal)
  }
}
