use backyard_nodes::{ cast_node, Node, NodeType, NodeWrapper };
use crate::generator::{ Builder, Generator, GeneratorArgument };

use super::call::CallGenerator;

pub struct AttributeGenerator;

impl AttributeGenerator {
  pub fn generate<'arena>(
    generator: &mut Generator<'arena, '_>,
    builder: &mut Builder,
    node: &Node<'arena>
  ) {
    let node = cast_node!(Attribute, &node.wrapper);
    builder.push("#[");
    let items = generator.generate_nodes_new(
      &node.items,
      &mut GeneratorArgument::for_parameter(&[(NodeType::AttributeItem, Self::generate_item)])
    );
    builder.push(&items.print(" "));
    builder.push("]");
  }

  pub fn generate_item<'arena>(
    generator: &mut Generator<'arena, '_>,
    builder: &mut Builder,
    node: &Node<'arena>
  ) {
    let node = cast_node!(AttributeItem, &node.wrapper);
    builder.push(&node.name);
    if !node.arguments.is_empty() {
      builder.push("(");
      let arguments = generator.generate_nodes_new(
        &node.arguments,
        &mut GeneratorArgument::for_parameter(
          &[(NodeType::CallArgument, CallGenerator::generate_argument)]
        )
      );
      builder.push(&arguments.print(" "));
      builder.push(")");
    }
  }
}
