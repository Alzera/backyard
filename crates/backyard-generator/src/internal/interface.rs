use backyard_nodes::{ cast_node, node::{ Node, NodeType, NodeWrapper } };

use crate::generator::{ Builder, Generator, GeneratorArgument };

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
    if node.extends.len() > 0 {
      builder.push(" extends ");
      let implements = generator.generate_nodes_new(
        &node.extends,
        &mut GeneratorArgument::for_parameter(
          &[(NodeType::Identifier, IdentifierGenerator::generate)]
        )
      );
      builder.push(&implements.to_string(" "));
    }
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
      "interface A extends B, C {
  const MY_CONSTANT = \"constant value\";
  public function a(int $x, int $y = 0): int;
}"
    );
  }
}
