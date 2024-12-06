use backyard_nodes::{ cast_node, node::{ Node, NodeType, NodeWrapper } };
use crate::generator::{ Builder, Generator, GeneratorArgument };

use super::call::CallGenerator;

pub struct AttributeGenerator;

impl AttributeGenerator {
  pub fn generate(generator: &mut Generator, builder: &mut Builder, node: &Box<Node>) {
    let node = cast_node!(Attribute, &node.node);

    builder.push("#[");
    let items = generator.generate_nodes_new(
      &node.items,
      &mut GeneratorArgument::for_parameter(&[(NodeType::AttributeItem, Self::generate_item)])
    );
    builder.push(&items.print(" "));
    builder.push("]");
  }

  pub fn generate_item(generator: &mut Generator, builder: &mut Builder, node: &Box<Node>) {
    let node = cast_node!(AttributeItem, &node.node);
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
