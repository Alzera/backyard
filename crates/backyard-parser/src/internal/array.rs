use bumpalo::collections::Vec;
use backyard_lexer::token::{ Token, TokenType };
use backyard_nodes::{
  node::{ ArrayItemNode, ArrayNode, Location, Node, NodeType },
  utils::{ IntoBoxedNode, IntoBoxedOptionNode },
};

use crate::{
  error::ParserError,
  guard,
  parser::{ LoopArgument, Parser, TokenTypeArrayCombine, DEFAULT_PARSERS },
  utils::{ match_pattern, Lookup, LookupResult },
};

#[derive(Debug, Clone)]
pub struct ArrayParser;

impl ArrayParser {
  pub fn get_values<'arena, 'b>(
    parser: &mut Parser<'arena, 'b>,
    breaker: TokenType
  ) -> Result<Vec<'arena, Node<'arena>>, ParserError> {
    let mut loop_parsers = DEFAULT_PARSERS.to_vec();
    loop_parsers.insert(0, (ArrayItemParser::test, ArrayItemParser::parse));
    Ok(
      Vec::from_iter_in(
        parser
          .get_children(
            &mut LoopArgument::new(
              parser.arena,
              "array",
              &[TokenType::Comma],
              &[breaker],
              &loop_parsers
            )
          )?
          .into_iter()
          .map(|i| {
            if i.node_type == NodeType::ArrayItem {
              i
            } else {
              let mut i = i;
              let leadings = i.leadings.take();
              let trailings = i.trailings.take();
              let loc = i.loc.take();
              let mut a = ArrayItemNode::loc(None, i.into_boxed(&parser.arena), loc);
              a.leadings = leadings;
              a.trailings = trailings;
              a
            }
          }),
        &parser.arena
      )
    )
  }

  pub fn test<'arena, 'a>(
    parser: &mut Parser<'arena, 'a>,
    tokens: &[Token],
    _: &mut LoopArgument
  ) -> Option<std::vec::Vec<LookupResult<'arena>>> {
    if
      let Some(m) = match_pattern(parser, tokens, &[Lookup::Equal(&[TokenType::LeftSquareBracket])])
    {
      return Some(m);
    }
    match_pattern(
      parser,
      tokens,
      &[Lookup::Equal(&[TokenType::Array]), Lookup::Equal(&[TokenType::LeftParenthesis])]
    )
  }

  pub fn parse<'arena, 'a, 'b>(
    parser: &mut Parser<'arena, 'a>,
    matched: std::vec::Vec<LookupResult>,
    start_loc: Location,
    _: &mut LoopArgument<'arena, 'b>
  ) -> Result<Node<'arena>, ParserError> {
    match matched.len() {
      1 => {
        if let [_] = matched.as_slice() {
          return Ok(
            ArrayNode::loc(
              true,
              ArrayParser::get_values(parser, TokenType::RightSquareBracket)?,
              parser.gen_loc(start_loc)
            )
          );
        }
      }
      2 => {
        if let [_, _] = matched.as_slice() {
          return Ok(
            ArrayNode::loc(
              false,
              ArrayParser::get_values(parser, TokenType::RightParenthesis)?,
              parser.gen_loc(start_loc)
            )
          );
        }
      }
      _ => {}
    }
    Err(ParserError::Internal)
  }
}

#[derive(Debug, Clone)]
pub struct ArrayItemParser;

impl ArrayItemParser {
  pub fn test<'arena, 'a>(
    parser: &mut Parser<'arena, 'a>,
    tokens: &[Token],
    _: &mut LoopArgument
  ) -> Option<std::vec::Vec<LookupResult<'arena>>> {
    match_pattern(parser, tokens, &[Lookup::Equal(&[TokenType::Arrow])])
  }

  pub fn parse<'arena, 'a, 'b>(
    parser: &mut Parser<'arena, 'a>,
    matched: std::vec::Vec<LookupResult>,
    start_loc: Location,
    args: &mut LoopArgument<'arena, 'b>
  ) -> Result<Node<'arena>, ParserError> {
    if let [_] = matched.as_slice() {
      let value = guard!(
        parser.get_statement(
          &mut LoopArgument::with_tokens(
            parser.arena,
            "array_item",
            &[],
            &args.breakers.combine(args.separators)
          )
        )?
      );
      let key = args.last_expr.take();
      return Ok(
        ArrayItemNode::loc(
          key.into_boxed(&parser.arena),
          value.into_boxed(&parser.arena),
          parser.gen_loc(start_loc)
        )
      );
    }
    Err(ParserError::Internal)
  }
}
