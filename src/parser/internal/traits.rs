use crate::{
  lexer::token::{ Token, TokenType },
  parser::{
    node::{ BlockNode, Node, TraitNode },
    parser::{ Internal, LoopArgument, Parser, ParserInternal },
    utils::{ match_pattern, Lookup },
  },
};

use super::{
  consts::ConstPropertyParser,
  identifier::IdentifierParser,
  method::MethodParser,
  property::PropertyParser,
  traituse::TraitUseParser,
};

#[derive(Debug, Clone)]
pub struct TraitParser {}

impl Internal for TraitParser {
  fn test(&self, tokens: &Vec<Token>, _: &LoopArgument) -> Option<Vec<Vec<Token>>> {
    match_pattern(
      tokens,
      [Lookup::Equal(vec![TokenType::Trait]), Lookup::Equal(vec![TokenType::Identifier])].to_vec()
    )
  }

  fn parse(&self, parser: &mut Parser, matched: Vec<Vec<Token>>, _: &LoopArgument) -> Option<Node> {
    if let [_, name] = matched.as_slice() {
      parser.position += 1;
      let body = parser.get_children(
        &mut LoopArgument::new(
          "trait_body",
          &[TokenType::Semicolon],
          &[TokenType::RightCurlyBracket],
          &[
            ParserInternal::TraitUse(TraitUseParser {}),
            ParserInternal::Property(PropertyParser {}),
            ParserInternal::Method(MethodParser {}),
            ParserInternal::ConstProperty(ConstPropertyParser {}),
          ]
        )
      );
      return Some(
        Box::new(TraitNode {
          name: IdentifierParser::from_matched(name),
          body: Box::new(BlockNode { statements: body }),
        })
      );
    }
    None
  }
}
