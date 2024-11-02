use crate::{
  guard,
  lexer::token::{ Token, TokenType },
  parser::{
    node::{ ForeachNode, Node },
    parser::{ Internal, LoopArgument, Parser },
    utils::{ match_pattern, Lookup },
  },
};

use super::block::BlockParser;

#[derive(Debug, Clone)]
pub struct ForeachParser {}

impl ForeachParser {
  fn get_key_value(parser: &mut Parser) -> Option<(Option<Node>, Node)> {
    let key_or_value = guard!(
      parser.get_statement(
        &mut LoopArgument::with_tokens(
          "foreach_key_or_value",
          &[],
          &[TokenType::Arrow, TokenType::RightParenthesis]
        )
      )
    );
    let has_key = guard!(parser.tokens.get(parser.position));
    parser.position += 1;
    if has_key.token_type == TokenType::RightParenthesis {
      return Some((None, key_or_value));
    } else if has_key.token_type == TokenType::Arrow {
      let value = guard!(
        parser.get_statement(
          &mut LoopArgument::with_tokens("foreach_value", &[], &[TokenType::RightParenthesis])
        )
      );
      parser.position += 1;
      return Some((Some(key_or_value), value));
    }
    None
  }
}

impl Internal for ForeachParser {
  fn test(&self, tokens: &Vec<Token>, _: &LoopArgument) -> Option<Vec<Vec<Token>>> {
    match_pattern(
      tokens,
      [
        Lookup::Equal(vec![TokenType::Foreach]),
        Lookup::Equal(vec![TokenType::LeftParenthesis]),
      ].to_vec()
    )
  }

  fn parse(&self, parser: &mut Parser, matched: Vec<Vec<Token>>, _: &LoopArgument) -> Option<Node> {
    if let [_, _] = matched.as_slice() {
      let source = guard!(
        parser.get_statement(
          &mut LoopArgument::with_tokens("foreach_source", &[], &[TokenType::As])
        )
      );
      parser.position += 1;
      if let Some((key, value)) = ForeachParser::get_key_value(parser) {
        let (is_short, body) = guard!(BlockParser::new_or_short(parser, &[TokenType::EndForeach]));
        return Some(
          Box::new(ForeachNode {
            source,
            key,
            value,
            body,
            is_short,
          })
        );
      }
    }
    None
  }
}
