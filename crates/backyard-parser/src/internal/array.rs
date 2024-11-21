use backyard_lexer::token::{ Token, TokenType, TokenTypeArrayCombine };
use backyard_nodes::node::{ Node, NodeType, ArrayItemNode, ArrayNode };
use utils::guard;

use crate::{
  error::ParserError,
  parser::{ LoopArgument, Parser, DEFAULT_PARSERS },
  utils::{ match_pattern, Lookup },
};

#[derive(Debug, Clone)]
pub struct ArrayParser {}

impl ArrayParser {
  pub fn get_values(
    parser: &mut Parser,
    breaker: TokenType
  ) -> Result<Vec<Box<Node>>, ParserError> {
    let mut loop_parsers = DEFAULT_PARSERS.to_vec();
    loop_parsers.insert(0, (ArrayItemParser::test, ArrayItemParser::parse));
    Ok(
      parser
        .get_children(
          &mut LoopArgument::new("array", &[TokenType::Comma], &[breaker], &loop_parsers)
        )?
        .iter()
        .map(|i| (
          if i.node_type == NodeType::ArrayItem {
            i.to_owned()
          } else {
            let mut i = i.to_owned();
            let leading_comments = i.leading_comments.to_owned();
            let trailing_comments = i.trailing_comments.to_owned();
            i.leading_comments = vec![];
            i.trailing_comments = vec![];
            let mut a = ArrayItemNode::new(None, i);
            a.leading_comments = leading_comments;
            a.trailing_comments = trailing_comments;
            a
          }
        ))
        .collect::<Vec<Box<Node>>>()
    )
  }

  pub fn test(tokens: &Vec<Token>, _: &mut LoopArgument) -> Option<Vec<Vec<Token>>> {
    if
      let Some(m) = match_pattern(
        tokens,
        [Lookup::Equal(vec![TokenType::LeftSquareBracket])].to_vec()
      )
    {
      return Some(m);
    }
    match_pattern(
      tokens,
      [
        Lookup::Equal(vec![TokenType::Array]),
        Lookup::Equal(vec![TokenType::LeftParenthesis]),
      ].to_vec()
    )
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<Vec<Token>>,
    args: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    match matched.len() {
      1 => {
        if let [_] = matched.as_slice() {
          return Ok(
            ArrayNode::new(true, ArrayParser::get_values(parser, TokenType::RightSquareBracket)?)
          );
        }
      }
      2 => {
        if let [_, _] = matched.as_slice() {
          return Ok(
            ArrayNode::new(false, ArrayParser::get_values(parser, TokenType::RightParenthesis)?)
          );
        }
      }
      _ => {}
    }
    Err(ParserError::internal("Array", args))
  }
}

#[derive(Debug, Clone)]
pub struct ArrayItemParser {}

impl ArrayItemParser {
  pub fn test(tokens: &Vec<Token>, _: &mut LoopArgument) -> Option<Vec<Vec<Token>>> {
    match_pattern(tokens, [Lookup::Equal(vec![TokenType::Arrow])].to_vec())
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<Vec<Token>>,
    args: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [_] = matched.as_slice() {
      let value = guard!(
        parser.get_statement(
          &mut LoopArgument::with_tokens("array_item", &[], &args.breakers.combine(args.separators))
        )?,
        {
          return Err(ParserError::internal("ArrayItem", args));
        }
      );
      let key = args.last_expr.to_owned();
      return Ok(ArrayItemNode::new(key, value));
    }
    Err(ParserError::internal("ArrayItem", args))
  }
}
