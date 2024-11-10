use crate::{
  generator::generator::{ Builder, Generator, GeneratorArgument },
  guard_ok,
  parser::{ node::{ Node, NodeTraitCast }, nodes::exit::ExitNode },
};

pub struct ExitGenerator {}

impl ExitGenerator {
  pub fn generate(generator: &mut Generator, builder: &mut Builder, node: &Node) {
    let node = guard_ok!(node.to_owned().cast::<ExitNode>(), {
      return;
    });
    builder.push("exit(");
    generator.generate_node(builder, &node.argument, &mut GeneratorArgument::default());
    builder.push(")");
  }
}
