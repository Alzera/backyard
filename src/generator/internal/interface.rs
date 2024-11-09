use crate::{
  generator::generator::{ Builder, Generator },
  guard_ok,
  parser::{ node::{ Node, NodeTraitCast }, nodes::interface::InterfaceNode },
};

use super::{ block::BlockGenerator, identifier::IdentifierGenerator };

pub struct InterfaceGenerator {}

impl InterfaceGenerator {
  pub fn generate(generator: &mut Generator, builder: &mut Builder, node: &Node) {
    let node = guard_ok!(node.to_owned().cast::<InterfaceNode>(), {
      return;
    });
    builder.push("interface ");
    IdentifierGenerator::generate(generator, builder, &node.name);
    BlockGenerator::generate(generator, builder, &node.body, None);
  }
}
