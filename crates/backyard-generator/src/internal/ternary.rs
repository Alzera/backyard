use backyard_nodes::{ cast_node, node::{ Node, NodeWrapper } };
use crate::generator::{ Builder, Generator, GeneratorArgument };

pub struct TernaryGenerator;

impl TernaryGenerator {
  pub fn generate(generator: &mut Generator, builder: &mut Builder, node: &Node) {
    let node = cast_node!(Ternary, &node.node);

    generator.generate_node(builder, &node.condition, &mut GeneratorArgument::default());
    builder.push(" ? ");
    generator.generate_node(builder, &node.valid, &mut GeneratorArgument::default());
    builder.push(" : ");
    generator.generate_node(builder, &node.invalid, &mut GeneratorArgument::default());
  }
}
