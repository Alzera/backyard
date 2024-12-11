use backyard_nodes::{ cast_node, node::{ Node, NodeWrapper } };

use crate::generator::{ Builder, Generator, GeneratorArgument };

pub struct ArrayLookupGenerator;

impl ArrayLookupGenerator {
  pub fn generate<'arena, 'a>(
    generator: &mut Generator<'arena, 'a>,
    builder: &mut Builder,
    node: &Node<'arena>
  ) {
    let node = cast_node!(ArrayLookup, &node.node);
    generator.generate_node(builder, &node.left, &mut GeneratorArgument::default());
    builder.push("[");
    if let Some(right) = &node.right {
      generator.generate_node(builder, right, &mut GeneratorArgument::default());
    }
    builder.push("]");
  }
}
