use backyard_nodes::{ cast_node, node::{ Node, NodeWrapper } };

use crate::generator::{ Builder, Generator };

pub struct IdentifierGenerator;

impl IdentifierGenerator {
  pub fn generate(_: &mut Generator, builder: &mut Builder, node: &Node) {
    let node = cast_node!(Identifier, &node.node);
    builder.push(&node.name);
  }
}
