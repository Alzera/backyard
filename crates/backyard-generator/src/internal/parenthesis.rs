use backyard_nodes::{ cast_node, node::{ Node, NodeWrapper } };

use crate::generator::{ Builder, Generator, GeneratorArgument };

pub struct ParenthesisGenerator;

impl ParenthesisGenerator {
  pub fn generate(generator: &mut Generator, builder: &mut Builder, node: &Box<Node>) {
    let node = cast_node!(Parenthesis, &node.node);
    builder.push("(");
    generator.generate_node(builder, &node.statement, &mut GeneratorArgument::default());
    builder.push(")");
  }

  pub fn generate_cast(generator: &mut Generator, builder: &mut Builder, node: &Box<Node>) {
    let node = cast_node!(Cast, &node.node);
    builder.push("(");
    builder.push(&node.cast_type);
    builder.push(") ");
    generator.generate_node(builder, &node.expression, &mut GeneratorArgument::default());
  }
}
