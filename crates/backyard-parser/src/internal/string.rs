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

use crate::{ error::ParserError, guard, parser::{ LocationHelper, LoopArgument, Parser } };

use super::variable::VariableParser;

#[derive(Debug, Clone)]
pub struct StringParser;

impl StringParser {
  pub fn test(tokens: &[Token], _: &mut LoopArgument) -> Option<Vec<Vec<Token>>> {
    if let Some(token) = tokens.first() {
      if
        [
          TokenType::EncapsedStringOpen,
          TokenType::String,
          TokenType::HeredocOpen,
          TokenType::NowDocOpen,
        ].contains(&token.token_type)
      {
        return Some(vec![vec![token.to_owned()]]);
      }
    }
    None
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<Vec<Token>>,
    start_loc: Location,
    args: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [string_type] = matched.as_slice() {
      if let Some(string_type) = string_type.first() {
        if string_type.token_type == TokenType::NowDocOpen {
          let label = string_type.value.to_owned();
          let text = guard!(parser.tokens.get(parser.position), {
            return Err(ParserError::internal("NowDoc", args));
          }).value.to_owned();
          if let Some(next) = parser.tokens.get(parser.position + 1) {
            if next.token_type == TokenType::NowDocClose {
              parser.position += 2;
              return Ok(NowDocNode::new(label, text, parser.gen_loc(start_loc)));
            }
          }
        } else if string_type.token_type == TokenType::HeredocOpen {
          let values = StringParser::parse_encapsed(parser, args, TokenType::HeredocClose)?;
          let label = string_type.value.to_owned();
          return Ok(HereDocNode::new(label, values, parser.gen_loc(start_loc)));
        } else if string_type.token_type == TokenType::EncapsedStringOpen {
          let values = StringParser::parse_encapsed(parser, args, TokenType::EncapsedStringClose)?;
          let quote = string_type.value.to_owned();
          return Ok(
            EncapsedNode::new(Quote::try_parse(&quote).unwrap(), values, parser.gen_loc(start_loc))
          );
        } else if string_type.token_type == TokenType::String {
          let mut value = string_type.value.to_owned();
          let quote = value.remove(0).to_string();
          value = value
            .get(..value.len() - 1)
            .unwrap_or_default()
            .to_owned();
          return Ok(
            StringNode::new(Quote::try_parse(&quote).unwrap(), value, parser.gen_loc(start_loc))
          );
        }
      }
    }
    Err(ParserError::internal("String", args))
  }

  #[allow(unused_variables, unreachable_patterns)]
  fn parse_encapsed(
    parser: &mut Parser,
    args: &mut LoopArgument,
    breaker: TokenType
  ) -> Result<Vec<Box<Node>>, ParserError> {
    let mut values: Vec<Box<Node>> = vec![];
    // let quote = open.value.to_owned();
    while let Some(i) = parser.tokens.get(parser.position) {
      let start_loc = i.get_location().unwrap();
      parser.position += 1;
      match i.token_type {
        c if c == breaker => {
          // if quote != i.value {
          //   return Err(ParserError::internal("StringEncapsed", args));
          // }
          break;
        }
        TokenType::EncapsedString =>
          values.push(
            EncapsedPartNode::new(
              false,
              StringNode::new(Quote::Single, i.value.to_owned(), parser.gen_loc(start_loc.clone())),
              parser.gen_loc(start_loc)
            )
          ),
        TokenType::Variable =>
          values.push(
            EncapsedPartNode::new(false, VariableParser::from_token(i), parser.gen_loc(start_loc))
          ),
        TokenType::AdvanceInterpolationOpen => {
          let value = guard!(
            parser.get_statement(
              &mut LoopArgument::with_tokens("string", &[TokenType::AdvanceInterpolationClose], &[])
            )?,
            {
              continue;
            }
          );
          parser.position += 1;
          values.push(EncapsedPartNode::new(true, value, parser.gen_loc(start_loc)));
        }
        _ => {
          continue;
        }
      }
    }
    Ok(values)
  }
}
