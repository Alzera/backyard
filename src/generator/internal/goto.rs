use crate::{
  generator::generator::{ Builder, Generator },
  guard_ok,
  parser::{ node::{ Node, NodeTraitCast }, nodes::goto::GotoNode },
};

use super::identifier::IdentifierGenerator;

pub struct GotoGenerator {}

impl GotoGenerator {
  pub fn generate(generator: &mut Generator, builder: &mut Builder, node: &Node) {
    let node = guard_ok!(node.to_owned().cast::<GotoNode>(), {
      return;
    });

    builder.push("goto ");
    IdentifierGenerator::generate(generator, builder, &node.label);
  }
}
