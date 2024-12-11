use backyard_nodes::{ cast_node, node::{ Node, NodeWrapper } };

use crate::generator::{ Builder, Generator, GeneratorArgument };

pub struct PostGenerator;

impl PostGenerator {
  pub fn generate<'arena>(
    generator: &mut Generator<'arena, '_>,
    builder: &mut Builder,
    node: &Node<'arena>
  ) {
    let node = cast_node!(Post, &node.node);
    generator.generate_node(builder, &node.statement, &mut GeneratorArgument::default());
    builder.push(node.operator.as_str());
  }
}
