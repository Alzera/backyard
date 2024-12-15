use backyard_lexer::token::TokenType;
use backyard_nodes::{ Location, Node, ObjectAccessNode, utils::IntoBoxedNode };

use crate::{
  error::ParserError,
  internal::{ identifier::IdentifierParser, variable::VariableParser },
  parser::{ LocationHelper, LoopArgument, OptionNodeOrInternal, Parser },
  utils::{ match_pattern, Lookup, LookupResult },
};

#[derive(Debug, Clone)]
pub struct ObjectAccessParser;

impl ObjectAccessParser {
  pub fn test<'arena, 'a>(
    parser: &mut Parser<'arena, 'a>,
    args: &mut LoopArgument
  ) -> Option<std::vec::Vec<LookupResult<'arena>>> {
    args.last_expr.as_ref()?;
    match_pattern(
      parser,
      &[Lookup::Equal(&[TokenType::ObjectAccess, TokenType::NullsafeObjectAccess])]
    )
  }

  pub fn parse<'arena, 'a, 'b>(
    parser: &mut Parser<'arena, 'a>,
    matched: std::vec::Vec<LookupResult<'arena>>,
    start_loc: Location,
    args: &mut LoopArgument<'arena, 'b>
  ) -> Result<Node<'arena>, ParserError> {
    if let [access_type] = matched.as_slice() {
      let is_nullsafe = access_type.as_equal(parser)?.token_type == TokenType::NullsafeObjectAccess;
      let is_bracket = parser.get_token(parser.position)?.token_type == TokenType::LeftCurlyBracket;
      let expr = if is_bracket {
        parser.position += 1;
        let t = parser
          .get_statement(
            &mut LoopArgument::with_tokens(
              parser.arena,
              "objectaccess",
              &[],
              &[TokenType::RightCurlyBracket]
            )
          )?
          .ok_internal()?;
        parser.position += 1;
        t
      } else if let Some(m) = VariableParser::test(parser, args) {
        let loc = parser.get_token(parser.position)?.get_location().unwrap();
        parser.position += m
          .iter()
          .map(|x| x.size)
          .sum::<usize>();
        VariableParser::parse(parser, m, loc, args)?
      } else if let Ok(token) = parser.get_token(parser.position) {
        let node = IdentifierParser::from_token(token);
        parser.position += 1;
        node
      } else {
        return Err(ParserError::Internal);
      };
      return Ok(
        ObjectAccessNode::loc(
          args.last_expr.take().unwrap().into_boxed(parser.arena),
          expr.into_boxed(parser.arena),
          is_bracket,
          is_nullsafe,
          parser.gen_loc(start_loc)
        )
      );
    }
    Err(ParserError::Internal)
  }
}
