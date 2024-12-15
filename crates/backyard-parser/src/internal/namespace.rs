use backyard_lexer::token::TokenType;
use backyard_nodes::{ BlockNode, Location, NamespaceNode, Node, utils::IntoBoxedNode };

use crate::{
  error::ParserError,
  parser::{ LocationHelper, LoopArgument, Parser },
  utils::{ match_pattern, Lookup, LookupResult },
};

#[derive(Debug, Clone)]
pub struct NamespaceParser;

impl NamespaceParser {
  pub fn test<'arena, 'a>(
    parser: &mut Parser<'arena, 'a>,
    _: &mut LoopArgument
  ) -> Option<std::vec::Vec<LookupResult<'arena>>> {
    match_pattern(
      parser,
      &[
        Lookup::Equal(&[TokenType::Namespace]),
        Lookup::Equal(
          &[
            TokenType::UnqualifiedName,
            TokenType::QualifiedName,
            TokenType::RelativeName,
            TokenType::FullyQualifiedName,
          ]
        ),
      ]
    )
  }

  pub fn parse<'arena, 'a, 'b>(
    parser: &mut Parser<'arena, 'a>,
    matched: std::vec::Vec<LookupResult<'arena>>,
    start_loc: Location,
    _: &mut LoopArgument<'arena, 'b>
  ) -> Result<Node<'arena>, ParserError> {
    if let [_, name] = matched.as_slice() {
      let name = name.as_equal(parser)?.value.to_owned();
      let is_bracket = parser.get_token(parser.position)?.token_type == TokenType::LeftCurlyBracket;
      let block_loc = parser.get_token(parser.position)?.get_location().unwrap();
      if is_bracket {
        parser.position += 1;
      }
      let body = BlockNode::loc(
        parser.get_children(&mut LoopArgument::default(parser.arena, "block_parser"))?,
        parser.gen_loc(block_loc)
      );
      return Ok(
        NamespaceNode::loc(
          name,
          body.into_boxed(parser.arena),
          is_bracket,
          parser.gen_loc(start_loc)
        )
      );
    }
    Err(ParserError::Internal)
  }
}
