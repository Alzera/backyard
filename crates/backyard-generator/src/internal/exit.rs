use backyard_nodes::{ cast_node, node::{ Node, NodeWrapper } };

use crate::generator::{ Builder, Generator, GeneratorArgument };

pub struct ExitGenerator;

impl ExitGenerator {
  pub fn generate(generator: &mut Generator, builder: &mut Builder, node: &Node) {
    let node = cast_node!(Exit, &node.node);
    builder.push("exit");
    if let Some(argument) = &node.statement {
      builder.push("(");
      generator.generate_node(builder, argument, &mut GeneratorArgument::default());
      builder.push(")");
    }
  }
}
