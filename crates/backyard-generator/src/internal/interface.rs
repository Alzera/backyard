use backyard_nodes::{ cast_node, Node, NodeType, NodeWrapper };

use crate::generator::{ Builder, Generator, GeneratorArgument };

use super::{
  block::BlockGenerator,
  consts::ConstGenerator,
  identifier::IdentifierGenerator,
  method::MethodGenerator,
};

pub struct InterfaceGenerator;

impl InterfaceGenerator {
  pub fn generate<'arena>(
    generator: &mut Generator<'arena, '_>,
    builder: &mut Builder,
    node: &Node<'arena>
  ) {
    let node = cast_node!(Interface, &node.wrapper);
    builder.push("interface ");
    IdentifierGenerator::generate(generator, builder, &node.name);
    if !node.extends.is_empty() {
      builder.push(" extends ");
      let implements = generator.generate_nodes_new(
        &node.extends,
        &mut GeneratorArgument::for_parameter(
          &[(NodeType::Identifier, IdentifierGenerator::generate)]
        )
      );
      builder.push(&implements.print(" "));
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
