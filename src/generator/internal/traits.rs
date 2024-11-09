use crate::{
  generator::generator::{ Builder, Generator },
  guard_ok,
  parser::{ node::{ Node, NodeTraitCast }, nodes::traits::TraitNode },
};

use super::{ block::BlockGenerator, identifier::IdentifierGenerator };

pub struct TraitGenerator {}

impl TraitGenerator {
  pub fn generate(generator: &mut Generator, builder: &mut Builder, node: &Node) {
    let node = guard_ok!(node.to_owned().cast::<TraitNode>(), {
      return;
    });
    builder.push("trait ");
    IdentifierGenerator::generate(generator, builder, &node.name);
    builder.push(" {");
    let mut block = Builder::new();
    BlockGenerator::generate(generator, &mut block, &node.body);
    block.indent();
    builder.extend(&block);
    builder.new_line();
    builder.push("}");
  }
}
