use backyard_nodes::{ cast_node, node::{ Node, NodeType, NodeWrapper } };

use crate::generator::{ Builder, Generator, GeneratorArgument };

pub struct StaticLookupGenerator;

impl StaticLookupGenerator {
  pub fn generate<'arena>(
    generator: &mut Generator<'arena, '_>,
    builder: &mut Builder,
    node: &Node<'arena>
  ) {
    let node = cast_node!(StaticLookup, &node.node);
    generator.generate_node(builder, &node.left, &mut GeneratorArgument::default());
    builder.push("::");
    if node.right.node_type == NodeType::ClassKeyword {
      builder.push("class");
    } else if node.use_bracket {
      builder.push("{");
      generator.generate_node(builder, &node.right, &mut GeneratorArgument::default());
      builder.push("}");
    } else {
      generator.generate_node(builder, &node.right, &mut GeneratorArgument::default());
    }
  }
}
