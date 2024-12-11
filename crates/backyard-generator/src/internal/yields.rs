use backyard_nodes::{ cast_node, node::{ Node, NodeWrapper } };

use crate::generator::{ Builder, Generator, GeneratorArgument };

pub struct YieldGenerator;

impl YieldGenerator {
  pub fn generate<'arena>(
    generator: &mut Generator<'arena, '_>,
    builder: &mut Builder,
    node: &Node<'arena>
  ) {
    let node = cast_node!(Yield, &node.wrapper);

    builder.push("yield");
    if let Some(key) = &node.key {
      builder.push(" ");
      generator.generate_node(builder, key, &mut GeneratorArgument::default());
      builder.push(" =>");
    }
    if let Some(value) = &node.value {
      builder.push(" ");
      generator.generate_node(builder, value, &mut GeneratorArgument::default());
    }
  }

  pub fn generate_from<'arena>(
    generator: &mut Generator<'arena, '_>,
    builder: &mut Builder,
    node: &Node<'arena>
  ) {
    let node = cast_node!(YieldFrom, &node.wrapper);

    builder.push("yield from ");
    generator.generate_node(builder, &node.statement, &mut GeneratorArgument::default());
  }
}
