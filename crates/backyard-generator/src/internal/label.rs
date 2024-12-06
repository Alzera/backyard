use backyard_nodes::{ cast_node, node::{ Node, NodeWrapper } };

use crate::generator::{ Builder, Generator };

use super::identifier::IdentifierGenerator;

pub struct LabelGenerator;

impl LabelGenerator {
  pub fn generate(generator: &mut Generator, builder: &mut Builder, node: &Box<Node>) {
    let node = cast_node!(Label, &node.node);

    IdentifierGenerator::generate(generator, builder, &node.label);
    builder.push(":");
  }
}
