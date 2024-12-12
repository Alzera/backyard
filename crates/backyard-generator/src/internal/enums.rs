use backyard_nodes::{ cast_node, Node, NodeType, NodeWrapper };

use crate::generator::{ Builder, EndMode, Generator, GeneratorArgument };

use super::{
  consts::ConstGenerator,
  identifier::IdentifierGenerator,
  method::MethodGenerator,
  types::TypeGenerator,
};

pub struct EnumGenerator;

impl EnumGenerator {
  pub fn generate<'arena>(
    generator: &mut Generator<'arena, '_>,
    builder: &mut Builder,
    node: &Node<'arena>
  ) {
    let node = cast_node!(Enum, &node.wrapper);
    builder.push("enum ");
    IdentifierGenerator::generate(generator, builder, &node.name);
    if let Some(n) = &node.enum_type {
      builder.push(": ");
      TypeGenerator::generate(generator, builder, n);
    }
    if let Some(n) = &node.implements {
      builder.push(" implements ");
      IdentifierGenerator::generate(generator, builder, n);
    }
    let mut body = generator.generate_nodes_new(
      &node.body,
      &mut GeneratorArgument::new(
        EndMode::SemicolonDynamic,
        &[
          (NodeType::ConstProperty, ConstGenerator::generate_property),
          (NodeType::Method, MethodGenerator::generate),
          (NodeType::EnumItem, Self::generate_item),
        ]
      )
    );
    builder.push(" {");
    body.indent();
    builder.extend(body);
    builder.new_line();
    builder.push("}");
  }

  pub fn generate_item<'arena>(
    generator: &mut Generator<'arena, '_>,
    builder: &mut Builder,
    node: &Node<'arena>
  ) {
    let node = cast_node!(EnumItem, &node.wrapper);
    builder.push("case ");
    generator.generate_node(builder, &node.value, &mut GeneratorArgument::default());
  }
}
