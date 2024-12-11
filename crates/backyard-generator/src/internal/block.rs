use backyard_nodes::node::{ Node, NodeType, NodeWrapper };

use crate::generator::{
  Builder,
  EndMode,
  Generator,
  GeneratorArgument,
  InternalGenerator,
  DEFAULT_GENERATORS,
};

pub struct BlockGenerator;

impl BlockGenerator {
  pub fn generate_single<'arena>(
    generator: &mut Generator<'arena, '_>,
    builder: &mut Builder,
    node: &Node<'arena>
  ) {
    builder.push("{");
    let mut block = Self::generate_base(generator, node, &DEFAULT_GENERATORS);
    block.indent();
    builder.extend(block);
    builder.new_line();
    builder.push("}");
  }

  pub fn generate<'arena>(
    generator: &mut Generator<'arena, '_>,
    builder: &mut Builder,
    node: &Node<'arena>,
    short_close: Option<&str>
  ) {
    Self::generate_specific(generator, builder, node, short_close, &DEFAULT_GENERATORS)
  }

  pub fn generate_specific<'arena>(
    generator: &mut Generator<'arena, '_>,
    builder: &mut Builder,
    node: &Node<'arena>,
    short_close: Option<&str>,
    generators: &[(NodeType, InternalGenerator)]
  ) {
    let leadings = &node.leadings;
    let trailings = &node.trailings;
    if
      (leadings.is_some() && !leadings.as_ref().unwrap().is_empty()) ||
      (trailings.is_some() && !trailings.as_ref().unwrap().is_empty())
    {
      let mut scoped_builder = Builder::new();
      if let Some(leadings) = leadings {
        generator.handle_comments(&mut scoped_builder, leadings);
      }
      if scoped_builder.total_len() == 0 {
        scoped_builder.new_line();
      }
      Self::print_block(generator, &mut scoped_builder, node, short_close, generators);
      if let Some(trailings) = trailings {
        generator.handle_comments(&mut scoped_builder, trailings);
      }
      builder.extend_first_line(scoped_builder);
    } else {
      Self::print_block(generator, builder, node, short_close, generators);
    }
  }

  fn print_block<'arena>(
    generator: &mut Generator<'arena, '_>,
    builder: &mut Builder,
    node: &Node<'arena>,
    short_close: Option<&str>,
    generators: &[(NodeType, InternalGenerator)]
  ) {
    let mut block = Self::generate_base(generator, node, generators);
    block.indent();
    if let Some(close) = short_close {
      builder.push(":");
      builder.extend(block);
      builder.new_line();
      builder.push(close);
    } else {
      if node.leadings.is_none() || node.leadings.as_ref().unwrap().is_empty() {
        builder.push(" ");
      }
      builder.push("{");
      builder.extend(block);
      builder.new_line();
      builder.push("}");
    }
  }

  pub fn generate_base<'arena>(
    generator: &mut Generator<'arena, '_>,
    node: &Node<'arena>,
    generators: &[(NodeType, InternalGenerator)]
  ) -> Builder {
    let mut block = Builder::new();
    let node = if let NodeWrapper::Block(n) = &node.wrapper {
      n
    } else {
      return block;
    };
    generator.generate_nodes(
      &mut block,
      &node.statements,
      &mut GeneratorArgument::new(EndMode::SemicolonDynamic, generators)
    );
    block
  }
}
