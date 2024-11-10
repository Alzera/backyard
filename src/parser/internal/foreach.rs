use crate::{
  guard_none,
  lexer::token::{ Token, TokenType },
  parser::{
    node::Node,
    nodes::foreach::ForeachNode,
    parser::{ LoopArgument, Parser },
    utils::{ match_pattern, Lookup },
  },
};

use super::block::BlockParser;

#[derive(Debug, Clone)]
pub struct ForeachParser {}

impl ForeachParser {
  fn get_key_value(parser: &mut Parser) -> Option<(Option<Node>, Node)> {
    let key_or_value = guard_none!(
      parser.get_statement(
        &mut LoopArgument::with_tokens(
          "foreach_key_or_value",
          &[],
          &[TokenType::Arrow, TokenType::RightParenthesis]
        )
      )
    );
    let has_key = guard_none!(parser.tokens.get(parser.position));
    parser.position += 1;
    if has_key.token_type == TokenType::RightParenthesis {
      return Some((None, key_or_value));
    } else if has_key.token_type == TokenType::Arrow {
      let value = guard_none!(
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

impl ForeachParser {
  pub fn test(tokens: &Vec<Token>, _: &mut LoopArgument) -> Option<Vec<Vec<Token>>> {
    match_pattern(
      tokens,
      [
        Lookup::Equal(vec![TokenType::Foreach]),
        Lookup::Equal(vec![TokenType::LeftParenthesis]),
      ].to_vec()
    )
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<Vec<Token>>,
    _: &mut LoopArgument
  ) -> Option<Node> {
    if let [_, _] = matched.as_slice() {
      let source = guard_none!(
        parser.get_statement(
          &mut LoopArgument::with_tokens("foreach_source", &[], &[TokenType::As])
        )
      );
      parser.position += 1;
      if let Some((key, value)) = ForeachParser::get_key_value(parser) {
        let (is_short, body) = guard_none!(
          BlockParser::new_or_short(parser, &[TokenType::EndForeach])
        );
        return Some(ForeachNode::new(source, key, value, body, is_short));
      }
    }
    None
  }
}
