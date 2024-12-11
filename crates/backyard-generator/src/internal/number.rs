use backyard_nodes::{ cast_node, node::{ Node, NodeWrapper } };

use crate::generator::{ Builder, Generator };

pub struct NumberGenerator;

impl NumberGenerator {
  pub fn generate(_: &mut Generator, builder: &mut Builder, node: &Node) {
    let node = cast_node!(Number, &node.wrapper);
    builder.push(&node.value);
  }
}
