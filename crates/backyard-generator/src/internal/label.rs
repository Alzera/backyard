use backyard_nodes::{ cast_node, node::{ Node, NodeWrapper } };

use crate::generator::{ Builder, Generator };

use super::identifier::IdentifierGenerator;

pub struct LabelGenerator;

impl LabelGenerator {
  pub fn generate<'arena>(
    generator: &mut Generator<'arena, '_>,
    builder: &mut Builder,
    node: &Node<'arena>
  ) {
    let node = cast_node!(Label, &node.wrapper);

    IdentifierGenerator::generate(generator, builder, &node.label);
    builder.push(":");
  }
}
