use bstr::BString;
use bumpalo::{ collections::Vec, vec };
use backyard_lexer::token::TokenType;
use backyard_nodes::{
  EncapsedNode,
  EncapsedPartNode,
  HereDocNode,
  Location,
  Node,
  NowDocNode,
  Quote,
  StringNode,
  utils::IntoBoxedNode,
};

use crate::{
  error::ParserError,
  guard,
  parser::{ LocationHelper, LoopArgument, Parser },
  utils::{ match_pattern, Lookup, LookupResult },
};

use super::variable::VariableParser;

#[derive(Debug, Clone)]
pub struct StringParser;

impl StringParser {
  pub fn test<'arena, 'a>(
    parser: &mut Parser<'arena, 'a>,
    _: &mut LoopArgument
  ) -> Option<std::vec::Vec<LookupResult<'arena>>> {
    match_pattern(
      parser,
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

  pub fn parse<'arena, 'a, 'b>(
    parser: &mut Parser<'arena, 'a>,
    matched: std::vec::Vec<LookupResult<'arena>>,
    start_loc: Location,
    args: &mut LoopArgument<'arena, 'b>
  ) -> Result<Node<'arena>, ParserError> {
    if let [string_type] = matched.as_slice() {
      let string_type = string_type.as_equal(parser)?;
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
        let label = string_type.value.to_owned();
        let values = StringParser::parse_encapsed(parser, args, TokenType::HeredocClose)?;
        return Ok(HereDocNode::loc(label, values, parser.gen_loc(start_loc)));
      } else if string_type.token_type == TokenType::EncapsedStringOpen {
        let quote = string_type.value.to_owned();
        let values = StringParser::parse_encapsed(parser, args, TokenType::EncapsedStringClose)?;
        return Ok(
          EncapsedNode::loc(
            Quote::try_from(&quote).map_err(|_| ParserError::Internal)?,
            values,
            parser.gen_loc(start_loc)
          )
        );
      } else if string_type.token_type == TokenType::String {
        let mut value = string_type.value.to_owned();
        let quote = BString::new(std::vec![value.remove(0)]);
        value.pop();
        return Ok(
          StringNode::loc(
            Quote::try_from(&quote).map_err(|_| ParserError::Internal)?,
            value,
            parser.gen_loc(start_loc)
          )
        );
      }
    }
    Err(ParserError::Internal)
  }

  #[allow(unused_variables, unreachable_patterns)]
  fn parse_encapsed<'arena, 'a, 'b>(
    parser: &mut Parser<'arena, 'a>,
    args: &mut LoopArgument<'arena, 'b>,
    breaker: TokenType
  ) -> Result<Vec<'arena, Node<'arena>>, ParserError> {
    let mut values = vec![in parser.arena];
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
              StringNode::loc(Quote::Single, i.value.to_owned(), loc.clone()).into_boxed(
                parser.arena
              ),
              loc
            )
          );
        }
        TokenType::Variable => {
          let parsed = VariableParser::from_token(parser.arena, i);
          let loc = parsed.loc.clone();
          values.push(EncapsedPartNode::loc(false, parsed.into_boxed(parser.arena), loc));
        }
        TokenType::AdvanceInterpolationOpen => {
          let value = guard!(
            parser.get_statement(
              &mut LoopArgument::with_tokens(
                parser.arena,
                "string",
                &[TokenType::AdvanceInterpolationClose],
                &[]
              )
            )?
          );
          parser.position += 1;
          values.push(
            EncapsedPartNode::loc(true, value.into_boxed(parser.arena), parser.gen_loc(start_loc))
          );
        }
        _ => {
          continue;
        }
      }
    }
    Ok(values)
  }
}
