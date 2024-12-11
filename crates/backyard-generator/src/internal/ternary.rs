use backyard_nodes::{ cast_node, node::{ Node, NodeWrapper } };
use crate::generator::{ Builder, Generator, GeneratorArgument };

pub struct TernaryGenerator;

impl TernaryGenerator {
  pub fn generate<'arena>(
    generator: &mut Generator<'arena, '_>,
    builder: &mut Builder,
    node: &Node<'arena>
  ) {
    let node = cast_node!(Ternary, &node.wrapper);

    generator.generate_node(builder, &node.condition, &mut GeneratorArgument::default());
    builder.push(" ? ");
    generator.generate_node(builder, &node.valid, &mut GeneratorArgument::default());
    builder.push(" : ");
    generator.generate_node(builder, &node.invalid, &mut GeneratorArgument::default());
  }
}
