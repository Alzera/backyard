use crate::{
  generator::generator::{ Builder, Generator },
  guard,
  parser::{ node::{ Node, NodeTraitCast, NodeType }, nodes::interface::InterfaceNode },
};

use super::{
  block::BlockGenerator,
  consts::ConstGenerator,
  identifier::IdentifierGenerator,
  method::MethodGenerator,
};

pub struct InterfaceGenerator {}

impl InterfaceGenerator {
  pub fn generate(generator: &mut Generator, builder: &mut Builder, node: &Node) {
    let node = guard!(node.to_owned().cast::<InterfaceNode>());
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
