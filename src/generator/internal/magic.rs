use crate::{
  generator::generator::{ Builder, Generator },
  guard_ok,
  parser::{ node::{ Node, NodeTraitCast }, nodes::magic::MagicNode },
};

pub struct MagicGenerator {}

impl MagicGenerator {
  pub fn generate(_: &mut Generator, builder: &mut Builder, node: &Node) {
    let node = guard_ok!(node.to_owned().cast::<MagicNode>(), {
      return;
    });
    builder.push(&node.name)
  }
}
