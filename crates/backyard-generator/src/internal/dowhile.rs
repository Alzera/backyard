use backyard_nodes::{ cast_node, Node, NodeWrapper };

use crate::generator::{ Builder, Generator, GeneratorArgument };

use super::block::BlockGenerator;

pub struct DoWhileGenerator;

impl DoWhileGenerator {
  pub fn generate<'arena>(
    generator: &mut Generator<'arena, '_>,
    builder: &mut Builder,
    node: &Node<'arena>
  ) {
    let node = cast_node!(DoWhile, &node.wrapper);

    builder.push("do");
    BlockGenerator::generate(generator, builder, &node.body, None);
    if node.body.trailings.is_none() || node.body.trailings.as_ref().unwrap().is_empty() {
      builder.push(" ");
    }
    generator.generate_node(builder, &node.condition, &mut GeneratorArgument::default());
  }

  pub fn generate_condition<'arena>(
    generator: &mut Generator<'arena, '_>,
    builder: &mut Builder,
    node: &Node<'arena>
  ) {
    let node = cast_node!(DoWhileCondition, &node.wrapper);

    builder.push("while (");
    generator.generate_node(builder, &node.condition, &mut GeneratorArgument::default());
    builder.push(");");
  }
}
