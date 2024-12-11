use backyard_nodes::{ cast_node, node::{ Node, NodeType, NodeWrapper } };

use crate::generator::{ Builder, Generator, GeneratorArgument, DEFAULT_GENERATORS };

use super::{
  block::BlockGenerator,
  consts::ConstGenerator,
  identifier::IdentifierGenerator,
  method::MethodGenerator,
  property::PropertyGenerator,
  traituse::TraitUseGenerator,
};

pub struct ClassGenerator;

impl ClassGenerator {
  pub fn generate<'arena>(
    generator: &mut Generator<'arena, '_>,
    builder: &mut Builder,
    node: &Node<'arena>
  ) {
    let node = cast_node!(Class, &node.node);
    if node.is_readonly {
      builder.push("readonly ");
    }
    if let Some(n) = &node.inheritance {
      builder.push(format!("{} ", n).as_str());
    }
    builder.push("class");
    if let Some(n) = &node.name {
      builder.push(" ");
      IdentifierGenerator::generate(generator, builder, n);
    }
    if let Some(n) = &node.extends {
      builder.push(" extends ");
      IdentifierGenerator::generate(generator, builder, n);
    }
    if !node.implements.is_empty() {
      builder.push(" implements ");
      let implements = generator.generate_nodes_new(
        &node.implements,
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
        (NodeType::TraitUse, TraitUseGenerator::generate),
        (NodeType::ConstProperty, ConstGenerator::generate_property),
        (NodeType::Property, PropertyGenerator::generate),
        (NodeType::Method, MethodGenerator::generate),
      ]
    );
  }

  pub fn generate_anonymous<'arena>(
    generator: &mut Generator<'arena, '_>,
    builder: &mut Builder,
    node: &Node<'arena>
  ) {
    let node = cast_node!(AnonymousClass, &node.node);
    builder.push("class");
    if !node.parameters.is_empty() {
      builder.push("(");
      let parameters = generator.generate_nodes_new(
        &node.parameters,
        &mut GeneratorArgument::for_parameter(&DEFAULT_GENERATORS)
      );
      builder.push(&parameters.print(" "));
      builder.push(")");
    }
    if let Some(n) = &node.extends {
      builder.push(" extends ");
      IdentifierGenerator::generate(generator, builder, n);
    }
    if !node.implements.is_empty() {
      builder.push(" implements ");
      let implements = generator.generate_nodes_new(
        &node.implements,
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
        (NodeType::TraitUse, TraitUseGenerator::generate),
        (NodeType::ConstProperty, ConstGenerator::generate_property),
        (NodeType::Property, PropertyGenerator::generate),
        (NodeType::Method, MethodGenerator::generate),
      ]
    );
  }
}
