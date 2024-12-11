use backyard_nodes::{ cast_node, node::{ Node, NodeWrapper } };

use crate::generator::{ Builder, Generator, GeneratorArgument };

use super::block::BlockGenerator;

pub struct DoWhileGenerator;

impl DoWhileGenerator {
  pub fn generate<'arena, 'a>(
    generator: &mut Generator<'arena, 'a>,
    builder: &mut Builder,
    node: &Node<'arena>
  ) {
    let node = cast_node!(DoWhile, &node.node);

    builder.push("do");
    BlockGenerator::generate(generator, builder, &node.body, None);
    if node.body.trailings.is_none() || node.body.trailings.as_ref().unwrap().is_empty() {
      builder.push(" ");
    }
    generator.generate_node(builder, &node.condition, &mut GeneratorArgument::default());
  }

  pub fn generate_condition<'arena, 'a>(
    generator: &mut Generator<'arena, 'a>,
    builder: &mut Builder,
    node: &Node<'arena>
  ) {
    let node = cast_node!(DoWhileCondition, &node.node);

    builder.push("while (");
    generator.generate_node(builder, &node.condition, &mut GeneratorArgument::default());
    builder.push(");");
  }
}
