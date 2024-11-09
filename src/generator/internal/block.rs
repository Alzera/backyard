use crate::{
  generator::generator::{
    Builder,
    EndMode,
    Generator,
    GeneratorArgument,
    InternalGenerator,
    DEFAULT_GENERATORS,
  },
  guard_ok,
  parser::{ node::{ Node, NodeTraitCast, NodeType }, nodes::block::BlockNode },
};

pub struct BlockGenerator {}

impl BlockGenerator {
  pub fn generate(generator: &mut Generator, builder: &mut Builder, node: &Node) {
    Self::generate_specific(generator, builder, node, &DEFAULT_GENERATORS)
  }

  pub fn generate_specific(
    generator: &mut Generator,
    builder: &mut Builder,
    node: &Node,
    generators: &[(NodeType, InternalGenerator)]
  ) {
    let node = guard_ok!(node.to_owned().cast::<BlockNode>(), {
      return;
    });
    let mut block = Builder::new();
    generator.generate_nodes(
      &mut block,
      &node.statements,
      &mut GeneratorArgument::new(EndMode::SemicolonDynamic, generators)
    );
    block.indent();
    builder.push(" {");
    builder.extend(&block);
    builder.new_line();
    builder.push("}");
  }
}
