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
    builder.push(" {");
    let mut block = Builder::new();
    BlockGenerator::generate(generator, &mut block, &node.body);
    block.indent();
    builder.extend(&block);
    builder.new_line();
    builder.push("}");
  }
}
