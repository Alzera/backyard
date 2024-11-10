use crate::{
  generator::generator::{ Builder, Generator, GeneratorArgument },
  guard_ok,
  parser::{ node::{ Node, NodeTraitCast }, nodes::eval::EvalNode },
};

pub struct EvalGenerator {}

impl EvalGenerator {
  pub fn generate(generator: &mut Generator, builder: &mut Builder, node: &Node) {
    let node = guard_ok!(node.to_owned().cast::<EvalNode>(), {
      return;
    });
    builder.push("eval(");
    generator.generate_node(builder, &node.argument, &mut GeneratorArgument::default());
    builder.push(")");
  }
}
