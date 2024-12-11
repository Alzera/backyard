use backyard_nodes::{ cast_node, node::{ Node, NodeWrapper } };

use crate::generator::{ Builder, Generator, DEFAULT_GENERATORS };

use super::block::BlockGenerator;

pub struct NamespaceGenerator;

impl NamespaceGenerator {
  pub fn generate<'arena>(
    generator: &mut Generator<'arena, '_>,
    builder: &mut Builder,
    node: &Node<'arena>
  ) {
    let node = cast_node!(Namespace, &node.wrapper);
    builder.push("namespace ");
    builder.push(&node.name);
    if node.is_bracket {
      BlockGenerator::generate(generator, builder, &node.body, None);
    } else {
      builder.push(";");
      let body = BlockGenerator::generate_base(generator, &node.body, &DEFAULT_GENERATORS);
      builder.extend(body);
    }
  }
}
