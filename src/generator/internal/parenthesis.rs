use crate::{
  generator::generator::{ Builder, Generator, GeneratorArgument },
  guard_ok,
  parser::{ node::{ Node, NodeTraitCast }, nodes::parenthesis::ParenthesisNode },
};

pub struct ParenthesisGenerator {}

impl ParenthesisGenerator {
  pub fn generate(generator: &mut Generator, builder: &mut Builder, node: &Node) {
    let node = guard_ok!(node.to_owned().cast::<ParenthesisNode>(), {
      return;
    });
    builder.push("(");
    generator.generate_node(builder, &node.statement, &mut GeneratorArgument::default());
    builder.push(")");
  }
}
