use crate::{
  generator::generator::{ Builder, Generator, GeneratorArgument },
  guard_ok,
  parser::{ node::{ Node, NodeTraitCast }, nodes::post::PostNode },
};

pub struct PostGenerator {}

impl PostGenerator {
  pub fn generate(generator: &mut Generator, builder: &mut Builder, node: &Node) {
    let node = guard_ok!(node.to_owned().cast::<PostNode>(), {
      return;
    });
    generator.generate_node(builder, &node.variable, &mut GeneratorArgument::default());
    builder.push(&node.operator.as_str());
  }
}
