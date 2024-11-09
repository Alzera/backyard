use crate::{
  generator::generator::{ Builder, Generator },
  guard_ok,
  parser::{ node::{ Node, NodeTraitCast }, nodes::identifier::IdentifierNode },
};

pub struct IdentifierGenerator {}

impl IdentifierGenerator {
  pub fn generate(_: &mut Generator, builder: &mut Builder, node: &Node) {
    let node = guard_ok!(node.to_owned().cast::<IdentifierNode>(), {
      return;
    });
    builder.push(&node.name);
  }
}
