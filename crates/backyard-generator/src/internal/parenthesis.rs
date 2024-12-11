use backyard_nodes::{ cast_node, node::{ Node, NodeWrapper } };

use crate::generator::{ Builder, Generator, GeneratorArgument };

pub struct ParenthesisGenerator;

impl ParenthesisGenerator {
  pub fn generate<'arena, 'a>(
    generator: &mut Generator<'arena, 'a>,
    builder: &mut Builder,
    node: &Node<'arena>
  ) {
    let node = cast_node!(Parenthesis, &node.node);
    builder.push("(");
    generator.generate_node(builder, &node.statement, &mut GeneratorArgument::default());
    builder.push(")");
  }

  pub fn generate_cast<'arena, 'a>(
    generator: &mut Generator<'arena, 'a>,
    builder: &mut Builder,
    node: &Node<'arena>
  ) {
    let node = cast_node!(Cast, &node.node);
    builder.push("(");
    builder.push(&node.cast_type);
    builder.push(") ");
    generator.generate_node(builder, &node.expression, &mut GeneratorArgument::default());
  }
}
