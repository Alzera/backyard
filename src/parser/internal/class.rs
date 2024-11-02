use crate::{
  lexer::token::{ Token, TokenType },
  parser::{
    node::{ BlockNode, ClassNode, Node },
    parser::{ Internal, LoopArgument, Parser, ParserInternal },
    utils::{ match_pattern, some_or_default, Lookup },
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
pub struct ClassParser {}

impl Internal for ClassParser {
  fn test(&self, tokens: &Vec<Token>, _: &LoopArgument) -> Option<Vec<Vec<Token>>> {
    match_pattern(
      tokens,
      [
        Lookup::Optional(vec![TokenType::Abstract, TokenType::Final]),
        Lookup::Equal(vec![TokenType::Class]),
        Lookup::Equal(vec![TokenType::Identifier]),
        Lookup::Optional(vec![TokenType::Extends]),
        Lookup::Optional(vec![TokenType::Identifier]),
        Lookup::Optional(vec![TokenType::Implements]),
      ].to_vec()
    )
  }

  fn parse(&self, parser: &mut Parser, matched: Vec<Vec<Token>>, _: &LoopArgument) -> Option<Node> {
    if let [modifier, _, name, _, extends, _] = matched.as_slice() {
      let implements = parser.get_children(
        &mut LoopArgument::new(
          "class_implements",
          &[TokenType::Comma],
          &[TokenType::LeftCurlyBracket],
          &[ParserInternal::Identifier(IdentifierParser {})]
        )
      );
      let body = parser.get_children(
        &mut LoopArgument::new(
          "class_body",
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
      let extends = match extends.len() {
        1 => Some(IdentifierParser::from_matched(extends)),
        _ => None,
      };
      return Some(
        Box::new(ClassNode {
          modifier: some_or_default(modifier.get(0), String::from(""), |i| i.value.to_owned()),
          name: IdentifierParser::from_matched(name),
          extends,
          implements,
          body: Box::new(BlockNode { statements: body }),
        })
      );
    }
    None
  }
}
