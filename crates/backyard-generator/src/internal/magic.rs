use backyard_nodes::{ cast_node, node::{ Node, NodeWrapper } };

use crate::generator::{ Builder, Generator };

pub struct MagicGenerator;

impl MagicGenerator {
  pub fn generate(_: &mut Generator, builder: &mut Builder, node: &Node) {
    let node = cast_node!(Magic, &node.wrapper);
    builder.push(&node.name)
  }
}
