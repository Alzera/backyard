use backyard_lexer::token::TokenType;
use backyard_nodes::{ Inheritance, Location, MethodNode, Node, Visibility, utils::IntoBoxedNode };

use crate::{
  error::ParserError,
  guard,
  parser::{ LoopArgument, Parser },
  utils::{ match_pattern, Lookup, LookupResult, ModifierLookup },
};

use super::{ comment::CommentParser, function::FunctionParser };

#[derive(Debug, Clone)]
pub struct MethodParser;

impl MethodParser {
  #[allow(unused_variables)]
  pub fn test<'arena, 'a>(
    parser: &mut Parser<'arena, 'a>,
    _: &mut LoopArgument
  ) -> Option<std::vec::Vec<LookupResult<'arena>>> {
    match_pattern(
      parser,
      &[
        Lookup::Modifiers(
          &[
            ModifierLookup::Custom(&[TokenType::Private, TokenType::Protected, TokenType::Public]),
            ModifierLookup::Custom(&[TokenType::Abstract, TokenType::Final]),
            ModifierLookup::Custom(&[TokenType::Static]),
          ]
        ),
        Lookup::Equal(&[TokenType::Function]),
      ]
    )
  }

  pub fn parse<'arena, 'a, 'b>(
    parser: &mut Parser<'arena, 'a>,
    matched: std::vec::Vec<LookupResult<'arena>>,
    start_loc: Location,
    _: &mut LoopArgument<'arena, 'b>
  ) -> Result<Node<'arena>, ParserError> {
    if let [modifiers, _] = matched.as_slice() {
      parser.position -= 1;
      let function = guard!(
        parser.get_statement(
          &mut LoopArgument::new(
            parser.arena,
            "method",
            &[TokenType::RightCurlyBracket],
            &[],
            &[
              (FunctionParser::test, FunctionParser::parse),
              (CommentParser::test, CommentParser::parse),
            ]
          )
        )?
      );
      let mut visibility = None;
      let mut inheritance = None;
      let mut is_static = false;
      if let Some([m0, m1, m2]) = modifiers.as_modifier() {
        visibility = m0.as_custom(parser, |x| Visibility::try_from(x));
        inheritance = m1.as_custom(parser, |x| Inheritance::try_from(x));
        is_static = m2.as_custom(parser, |x| Ok(x == "static")).unwrap_or(false);
      }
      return Ok(
        MethodNode::loc(
          visibility,
          inheritance,
          is_static,
          function.into_boxed(parser.arena),
          parser.gen_loc(start_loc)
        )
      );
    }
    Err(ParserError::Internal)
  }
}
