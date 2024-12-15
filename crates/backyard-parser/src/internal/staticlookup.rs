use backyard_lexer::token::TokenType;
use backyard_nodes::{ ClassKeywordNode, Location, Node, StaticLookupNode, utils::IntoBoxedNode };

use crate::{
  error::ParserError,
  parser::{ LocationHelper, LoopArgument, Parser },
  utils::{ match_pattern, Lookup, LookupResult },
};

use super::{ identifier::IdentifierParser, variable::VariableParser };

#[derive(Debug, Clone)]
pub struct StaticLookupParser;

impl StaticLookupParser {
  pub fn test<'arena, 'a>(
    parser: &mut Parser<'arena, 'a>,
    args: &mut LoopArgument
  ) -> Option<std::vec::Vec<LookupResult<'arena>>> {
    args.last_expr.as_ref()?;
    match_pattern(parser, &[Lookup::Equal(&[TokenType::DoubleColon])])
  }

  pub fn parse<'arena, 'a, 'b>(
    parser: &mut Parser<'arena, 'a>,
    matched: std::vec::Vec<LookupResult<'arena>>,
    start_loc: Location,
    args: &mut LoopArgument<'arena, 'b>
  ) -> Result<Node<'arena>, ParserError> {
    if let [_] = matched.as_slice() {
      let left = args.last_expr.take().unwrap();
      args.last_expr = None;
      if let Some(t) = parser.tokens.get(parser.position) {
        let expr = if t.token_type == TokenType::Class {
          parser.position += 1;
          ClassKeywordNode::loc(parser.gen_loc_helper(t))
        } else if [TokenType::Variable, TokenType::VariableBracketOpen].contains(&t.token_type) {
          if let Some(m) = VariableParser::test(parser, args) {
            parser.position += 1;
            VariableParser::parse(parser, m, t.get_location().unwrap(), args)?
          } else {
            return Err(ParserError::Internal);
          }
        } else if t.token_type == TokenType::LeftCurlyBracket {
          parser.position += 1;
          let expr = parser.get_statement(
            &mut LoopArgument::with_tokens(
              parser.arena,
              "staticlookup",
              &[],
              &[TokenType::RightCurlyBracket]
            )
          )?;
          parser.position += 1;
          if let Some(expr) = expr {
            return Ok(
              StaticLookupNode::loc(
                left.into_boxed(parser.arena),
                expr.into_boxed(parser.arena),
                true,
                parser.gen_loc(start_loc)
              )
            );
          } else {
            return Err(ParserError::Internal);
          }
        } else {
          parser.position += 1;
          IdentifierParser::from_token(t)
        };
        return Ok(
          StaticLookupNode::loc(
            left.into_boxed(parser.arena),
            expr.into_boxed(parser.arena),
            false,
            parser.gen_loc(start_loc)
          )
        );
      };
    }
    Err(ParserError::Internal)
  }
}
