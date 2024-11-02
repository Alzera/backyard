use crate::{
  guard,
  lexer::token::{ Token, TokenType },
  parser::{
    node::{ IfNode, Node },
    parser::{ Internal, LoopArgument, Parser },
    utils::{ match_pattern, Lookup },
  },
};

use super::block::BlockParser;

#[derive(Debug, Clone)]
pub struct IfParser {}

impl Internal for IfParser {
  fn test(&self, tokens: &Vec<Token>, _: &LoopArgument) -> Option<Vec<Vec<Token>>> {
    match_pattern(
      tokens,
      [Lookup::Equal(vec![TokenType::If]), Lookup::Equal(vec![TokenType::LeftParenthesis])].to_vec()
    )
  }

  fn parse(
    &self,
    parser: &mut Parser,
    matched: Vec<Vec<Token>>,
    args: &LoopArgument
  ) -> Option<Node> {
    if let [_, _] = matched.as_slice() {
      let condition = guard!(
        parser.get_statement(
          &mut LoopArgument::with_tokens("if", &[], &[TokenType::RightParenthesis])
        )
      );
      parser.position += 1;

      let next_token = guard!(parser.tokens.get(parser.position));
      let mut is_short = false;
      let valid = match next_token.token_type {
        TokenType::ShortFormStart => {
          is_short = true;
          let parsed = BlockParser::new_short(
            parser,
            &[TokenType::ElseIf, TokenType::Else, TokenType::EndIf]
          );
          parser.position -= 1;
          parsed
        }
        TokenType::LeftCurlyBracket => BlockParser::new(parser),
        _ => {
          let parsed = guard!(
            parser.get_statement(
              &mut LoopArgument::with_tokens("if_body", &[], &[TokenType::Semicolon])
            )
          );
          parser.position += 1;
          parsed
        }
      };
      let mut invalid = None;
      if let Some(next_token) = parser.tokens.get(parser.position) {
        match next_token.token_type {
          TokenType::Else => {
            parser.position += 1;
            if let Some(next_token) = parser.tokens.get(parser.position) {
              match next_token.token_type {
                TokenType::LeftCurlyBracket | TokenType::ShortFormStart | TokenType::ShortForm => {
                  if is_short {
                    invalid = Some(BlockParser::new_short(parser, &[TokenType::EndIf]));
                  } else {
                    invalid = Some(BlockParser::new(parser));
                  }
                }
                TokenType::If => {
                  invalid = parser.get_statement(&mut LoopArgument::default("if_invalid"));
                }
                _ => {
                  invalid = parser.get_statement(
                    &mut LoopArgument::with_tokens(
                      "if_else",
                      &[],
                      &[TokenType::Semicolon, TokenType::Else, TokenType::EndIf]
                    )
                  );
                }
              };
            }
          }
          TokenType::ElseIf => {
            parser.position += 2;
            invalid = (IfParser {}).parse(parser, vec![vec![], vec![]], args);
          }
          _ => {}
        }
      }
      return Some(
        Box::new(IfNode {
          condition,
          valid,
          invalid,
          is_short,
        })
      );
    }
    None
  }
}
