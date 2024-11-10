use crate::{
  generator::generator::{ Builder, Generator },
  guard_ok,
  parser::{ node::{ Node, NodeTraitCast }, nodes::label::LabelNode },
};

use super::identifier::IdentifierGenerator;

pub struct LabelGenerator {}

impl LabelGenerator {
  pub fn generate(generator: &mut Generator, builder: &mut Builder, node: &Node) {
    let node = guard_ok!(node.to_owned().cast::<LabelNode>(), {
      return;
    });

    IdentifierGenerator::generate(generator, builder, &node.label);
    builder.push(":");
  }
}
