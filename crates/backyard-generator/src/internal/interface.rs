use backyard_nodes::{ cast_node, node::{ Node, NodeType, NodeWrapper } };

use crate::generator::{ Builder, Generator };

use super::{
  block::BlockGenerator,
  consts::ConstGenerator,
  identifier::IdentifierGenerator,
  method::MethodGenerator,
};

pub struct InterfaceGenerator {}

impl InterfaceGenerator {
  pub fn generate(generator: &mut Generator, builder: &mut Builder, node: &Box<Node>) {
    let node = cast_node!(NodeWrapper::Interface, &node.node);
    builder.push("interface ");
    IdentifierGenerator::generate(generator, builder, &node.name);
    BlockGenerator::generate_specific(
      generator,
      builder,
      &node.body,
      None,
      &[
        (NodeType::ConstProperty, ConstGenerator::generate_property),
        (NodeType::Method, MethodGenerator::generate),
      ]
    );
  }
}

#[cfg(test)]
mod tests {
  use crate::test_utils::test;

  #[test]
  fn basic() {
    test("interface A {\n}");
    test(
      "interface A {
  const MY_CONSTANT = \"constant value\";
  public function a(int $x, int $y = 0): int;
}"
    );
  }
}
