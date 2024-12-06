use backyard_lexer::token::{ Token, TokenType };
use backyard_nodes::node::{ ArrayItemNode, ArrayNode, Location, Node, NodeType };

use crate::{
  error::ParserError,
  guard,
  parser::{ LoopArgument, Parser, TokenTypeArrayCombine, DEFAULT_PARSERS },
  utils::{ match_pattern, Lookup, LookupResult },
};

#[derive(Debug, Clone)]
pub struct ArrayParser;

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
            let leadings = i.leadings.to_owned();
            let trailings = i.trailings.to_owned();
            let loc = i.loc.to_owned();
            i.leadings = vec![];
            i.trailings = vec![];
            let mut a = ArrayItemNode::loc(None, i, loc);
            a.leadings = leadings;
            a.trailings = trailings;
            a
          }
        ))
        .collect::<Vec<Box<Node>>>()
    )
  }

  pub fn test(tokens: &[Token], _: &mut LoopArgument) -> Option<Vec<LookupResult>> {
    if let Some(m) = match_pattern(tokens, &[Lookup::Equal(&[TokenType::LeftSquareBracket])]) {
      return Some(m);
    }
    match_pattern(
      tokens,
      &[Lookup::Equal(&[TokenType::Array]), Lookup::Equal(&[TokenType::LeftParenthesis])]
    )
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<LookupResult>,
    start_loc: Location,
    _: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
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
  pub fn test(tokens: &[Token], _: &mut LoopArgument) -> Option<Vec<LookupResult>> {
    match_pattern(tokens, &[Lookup::Equal(&[TokenType::Arrow])])
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<LookupResult>,
    start_loc: Location,
    args: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [_] = matched.as_slice() {
      let value = guard!(
        parser.get_statement(
          &mut LoopArgument::with_tokens("array_item", &[], &args.breakers.combine(args.separators))
        )?
      );
      let key = args.last_expr.to_owned();
      return Ok(ArrayItemNode::loc(key, value, parser.gen_loc(start_loc)));
    }
    Err(ParserError::Internal)
  }
}
