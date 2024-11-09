use crate::{
  generator::generator::{ Builder, Generator, GeneratorArgument },
  guard_ok,
  parser::{ node::{ Node, NodeTraitCast }, nodes::block::BlockNode },
};

pub struct BlockGenerator {}

impl BlockGenerator {
  pub fn generate(generator: &mut Generator, builder: &mut Builder, node: &Node) {
    let node = guard_ok!(node.to_owned().cast::<BlockNode>(), {
      return;
    });
    generator.generate_nodes(builder, &node.statements, &mut GeneratorArgument::for_block());
  }
}
