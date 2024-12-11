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
  pub fn test<'arena, 'a>(
    parser: &mut Parser<'arena, 'a>,
    tokens: &[Token],
    _: &mut LoopArgument
  ) -> Option<std::vec::Vec<LookupResult<'arena>>> {
    match_pattern(
      parser,
      tokens,
      &[Lookup::Equal(&[TokenType::CommentLine, TokenType::CommentBlock, TokenType::CommentDoc])]
    )
  }

  pub fn parse<'arena, 'a, 'b>(
    parser: &mut Parser<'arena, 'a>,
    matched: std::vec::Vec<LookupResult>,
    start_loc: Location,
    args: &mut LoopArgument<'arena, 'b>
  ) -> Result<Node<'arena>, ParserError> {
    if let [comment] = matched.as_slice() {
      let comment = comment.as_equal()?;
      let comment = match comment.token_type {
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
        &mut LoopArgument::safe(
          parser.arena,
          "comment",
          args.separators,
          args.breakers,
          args.parsers
        )
      )?;
      if let Some(test_expr) = &expr {
        if
          ![NodeType::CommentBlock, NodeType::CommentDoc, NodeType::CommentLine].contains(
            &test_expr.node_type
          )
        {
          let mut expr = expr.unwrap();
          if let Some(expr_loc) = &expr.loc {
            if let Some(comment_loc) = &comment.loc {
              if expr_loc.start.offset < comment_loc.start.offset {
                expr.trailings_push(&parser.arena, comment);
              } else {
                expr.leadings_shift(&parser.arena, comment);
              }
            }
          }
          return Ok(expr);
        }
      }
      if let Some(mut last_expr) = args.last_expr.take() {
        if
          ![NodeType::CommentBlock, NodeType::CommentDoc, NodeType::CommentLine].contains(
            &last_expr.node_type
          )
        {
          last_expr.trailings_push(&parser.arena, comment);
          if let Some(next_expr) = expr {
            if
              [NodeType::CommentBlock, NodeType::CommentDoc, NodeType::CommentLine].contains(
                &next_expr.node_type
              )
            {
              last_expr.trailings_push(&parser.arena, next_expr);
            }
          }
          return Ok(last_expr);
        }
      }
      if let Some(statements) = &mut args.statements {
        if let Some(expr) = statements.pop() {
          if
            ![NodeType::CommentBlock, NodeType::CommentDoc, NodeType::CommentLine].contains(
              &expr.node_type
            )
          {
            return Ok(expr);
          }
        }
      }
      return Ok(comment);
    }
    Err(ParserError::Internal)
  }
}
