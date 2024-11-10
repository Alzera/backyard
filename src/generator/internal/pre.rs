use crate::{
  generator::generator::{ Builder, Generator, GeneratorArgument },
  guard_ok,
  parser::{ node::{ Node, NodeTraitCast }, nodes::pre::PreNode },
};

pub struct PreGenerator {}

impl PreGenerator {
  pub fn generate(generator: &mut Generator, builder: &mut Builder, node: &Node) {
    let node = guard_ok!(node.to_owned().cast::<PreNode>(), {
      return;
    });
    builder.push(&node.operator.as_str());
    generator.generate_node(builder, &node.variable, &mut GeneratorArgument::default());
  }
}
