use backyard_lexer::token::{ Token, TokenType };
use backyard_nodes::node::{
  EncapsedNode,
  EncapsedPartNode,
  HereDocNode,
  Location,
  Node,
  NowDocNode,
  Quote,
  StringNode,
};
use compact_str::ToCompactString;

use crate::{
  error::ParserError,
  guard,
  parser::{ LocationHelper, LoopArgument, Parser },
  utils::{ match_pattern, Lookup, LookupResult, LookupResultWrapper },
};

use super::variable::VariableParser;

#[derive(Debug, Clone)]
pub struct StringParser;

impl StringParser {
  pub fn test(tokens: &[Token], _: &mut LoopArgument) -> Option<Vec<LookupResult>> {
    match_pattern(
      tokens,
      &[
        Lookup::Equal(
          &[
            TokenType::EncapsedStringOpen,
            TokenType::String,
            TokenType::HeredocOpen,
            TokenType::NowDocOpen,
          ]
        ),
      ]
    )
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<LookupResult>,
    start_loc: Location,
    args: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [string_type] = matched.as_slice() {
      if let LookupResultWrapper::Equal(string_type) = &string_type.wrapper {
        if string_type.token_type == TokenType::NowDocOpen {
          let label = string_type.value.to_owned();
          let text = guard!(parser.tokens.get(parser.position)).value.to_owned();
          if let Some(next) = parser.tokens.get(parser.position + 1) {
            if next.token_type == TokenType::NowDocClose {
              parser.position += 2;
              return Ok(NowDocNode::loc(label, text, parser.gen_loc(start_loc)));
            }
          }
        } else if string_type.token_type == TokenType::HeredocOpen {
          let values = StringParser::parse_encapsed(parser, args, TokenType::HeredocClose)?;
          let label = string_type.value.to_owned();
          return Ok(HereDocNode::loc(label, values, parser.gen_loc(start_loc)));
        } else if string_type.token_type == TokenType::EncapsedStringOpen {
          let values = StringParser::parse_encapsed(parser, args, TokenType::EncapsedStringClose)?;
          let quote = string_type.value.to_owned();
          return Ok(
            EncapsedNode::loc(Quote::try_parse(&quote).unwrap(), values, parser.gen_loc(start_loc))
          );
        } else if string_type.token_type == TokenType::String {
          let mut value = string_type.value.to_owned();
          let quote = value.remove(0).to_string();
          value = value
            .get(..value.len() - 1)
            .unwrap_or_default()
            .to_compact_string();
          return Ok(
            StringNode::loc(Quote::try_parse(&quote).unwrap(), value, parser.gen_loc(start_loc))
          );
        }
      }
    }
    Err(ParserError::Internal)
  }

  #[allow(unused_variables, unreachable_patterns)]
  fn parse_encapsed(
    parser: &mut Parser,
    args: &mut LoopArgument,
    breaker: TokenType
  ) -> Result<Vec<Box<Node>>, ParserError> {
    let mut values: Vec<Box<Node>> = vec![];
    while let Some(i) = parser.tokens.get(parser.position) {
      let start_loc = i.get_location().unwrap();
      parser.position += 1;
      match i.token_type {
        c if c == breaker => {
          break;
        }
        TokenType::EncapsedString => {
          let loc = i.get_range_location();
          values.push(
            EncapsedPartNode::loc(
              false,
              StringNode::loc(Quote::Single, i.value.to_owned(), loc.clone()),
              loc
            )
          );
        }
        TokenType::Variable => {
          let parsed = VariableParser::from_token(i);
          let loc = parsed.loc.clone();
          values.push(EncapsedPartNode::loc(false, parsed, loc));
        }
        TokenType::AdvanceInterpolationOpen => {
          let value = guard!(
            parser.get_statement(
              &mut LoopArgument::with_tokens("string", &[TokenType::AdvanceInterpolationClose], &[])
            )?
          );
          parser.position += 1;
          values.push(EncapsedPartNode::loc(true, value, parser.gen_loc(start_loc)));
        }
        _ => {
          continue;
        }
      }
    }
    Ok(values)
  }
}
