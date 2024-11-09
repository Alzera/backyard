use crate::{
  generator::generator::{ Builder, Generator, GeneratorArgument },
  guard_ok,
  parser::{ node::{ Node, NodeTraitCast }, nodes::dowhile::DoWhileNode },
};

use super::block::BlockGenerator;

pub struct DoWhileGenerator {}

impl DoWhileGenerator {
  pub fn generate(generator: &mut Generator, builder: &mut Builder, node: &Node) {
    let node = guard_ok!(node.to_owned().cast::<DoWhileNode>(), {
      return;
    });

    builder.push("do");
    BlockGenerator::generate(generator, builder, &node.body, None);
    builder.push(" while (");
    generator.generate_node(builder, &node.condition, &mut GeneratorArgument::default());
    builder.push(");");
  }
}
