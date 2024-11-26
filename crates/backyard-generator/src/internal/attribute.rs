use backyard_nodes::{ cast_node, node::{ Node, NodeType, NodeWrapper } };
use crate::generator::{ Builder, Generator, GeneratorArgument, DEFAULT_GENERATORS };

pub struct AttributeGenerator;

impl AttributeGenerator {
  pub fn generate(generator: &mut Generator, builder: &mut Builder, node: &Box<Node>) {
    let node = cast_node!(NodeWrapper::Attribute, &node.node);

    builder.push("#[");
    let items = generator.generate_nodes_new(
      &node.items,
      &mut GeneratorArgument::for_parameter(&[(NodeType::AttributeItem, Self::generate_item)])
    );
    builder.push(&items.to_string(" "));
    builder.push("]");
  }

  pub fn generate_item(generator: &mut Generator, builder: &mut Builder, node: &Box<Node>) {
    let node = cast_node!(NodeWrapper::AttributeItem, &node.node);
    builder.push(&node.name);
    if !node.arguments.is_empty() {
      builder.push("(");
      let arguments = generator.generate_nodes_new(
        &node.arguments,
        &mut GeneratorArgument::for_parameter(&DEFAULT_GENERATORS)
      );
      builder.push(&arguments.to_string(" "));
      builder.push(")");
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::test_utils::test_eval;

  #[test]
  fn basic() {
    test_eval(
      "#[Attr]
#[\\MyExample\\MyAttribute]
#[Attr(123)]
#[\\Attr(123)]
#[Attr(123), \\Attr(123)]
class A {
}"
    );
  }
}