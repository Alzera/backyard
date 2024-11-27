use backyard_lexer::token::{ Token, TokenType };
use backyard_nodes::node::{ BlockNode, Location, Node, TraitNode };

use crate::{
  error::ParserError,
  parser::{ LocationHelper, LoopArgument, Parser },
  utils::{ match_pattern, Lookup },
};

use super::{
  attribute::AttributeParser,
  comment::CommentParser,
  consts::ConstPropertyParser,
  identifier::IdentifierParser,
  method::MethodParser,
  property::PropertyParser,
  traituse::TraitUseParser,
};

#[derive(Debug, Clone)]
pub struct TraitParser;

impl TraitParser {
  pub fn test(tokens: &[Token], _: &mut LoopArgument) -> Option<Vec<Vec<Token>>> {
    match_pattern(
      tokens,
      &[Lookup::Equal(&[TokenType::Trait]), Lookup::Equal(&[TokenType::Identifier])]
    )
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<Vec<Token>>,
    start_loc: Location,
    args: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [_, name] = matched.as_slice() {
      let block_loc = parser.tokens.get(parser.position).unwrap().get_location().unwrap();
      parser.position += 1;
      let body = parser.get_children(
        &mut LoopArgument::new(
          "trait_body",
          &[TokenType::Semicolon],
          &[TokenType::RightCurlyBracket],
          &[
            (TraitUseParser::test, TraitUseParser::parse),
            (MethodParser::test, MethodParser::parse),
            (ConstPropertyParser::test, ConstPropertyParser::parse),
            (PropertyParser::test, PropertyParser::parse),
            (AttributeParser::test, AttributeParser::parse),
            (CommentParser::test, CommentParser::parse),
          ]
        )
      )?;
      return Ok(
        TraitNode::new(
          IdentifierParser::from_matched(name),
          BlockNode::new(body, parser.gen_loc(block_loc)),
          parser.gen_loc(start_loc)
        )
      );
    }
    Err(ParserError::internal("Trait", args))
  }
}
