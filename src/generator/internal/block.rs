use crate::{
  generator::generator::{
    Builder,
    EndMode,
    Generator,
    GeneratorArgument,
    InternalGenerator,
    DEFAULT_GENERATORS,
  },
  guard,
  parser::{ node::{ Node, NodeTraitCast, NodeType }, nodes::block::BlockNode },
};

pub struct BlockGenerator {}

impl BlockGenerator {
  pub fn generate(
    generator: &mut Generator,
    builder: &mut Builder,
    node: &Node,
    short_close: Option<&str>
  ) {
    Self::generate_specific(generator, builder, node, short_close, &DEFAULT_GENERATORS)
  }

  pub fn generate_specific(
    generator: &mut Generator,
    builder: &mut Builder,
    node: &Node,
    short_close: Option<&str>,
    generators: &[(NodeType, InternalGenerator)]
  ) {
    let mut block = Self::generate_base(generator, node, generators);
    block.indent();

    if let Some(close) = short_close {
      builder.push(":");
      builder.extend(&block);
      builder.new_line();
      builder.push(close);
    } else {
      builder.push(" {");
      builder.extend(&block);
      builder.new_line();
      builder.push("}");
    }
  }

  pub fn generate_base(
    generator: &mut Generator,
    node: &Node,
    generators: &[(NodeType, InternalGenerator)]
  ) -> Builder {
    let mut block = Builder::new();
    let node = guard!(node.to_owned().cast::<BlockNode>(), {
      return block;
    });
    generator.generate_nodes(
      &mut block,
      &node.statements,
      &mut GeneratorArgument::new(EndMode::SemicolonDynamic, generators)
    );
    block
  }
}
