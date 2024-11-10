use crate::{
  generator::generator::{ Builder, Generator },
  guard,
  parser::{ node::{ Node, NodeTraitCast }, nodes::identifier::IdentifierNode },
};

pub struct IdentifierGenerator {}

impl IdentifierGenerator {
  pub fn generate(_: &mut Generator, builder: &mut Builder, node: &Node) {
    let node = guard!(node.to_owned().cast::<IdentifierNode>());
    builder.push(&node.name);
  }
}
