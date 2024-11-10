use crate::{
  generator::generator::{ Builder, Generator, GeneratorArgument },
  guard_ok,
  parser::{ node::{ Node, NodeTraitCast, NodeType }, nodes::class::ClassNode },
};

use super::{
  block::BlockGenerator,
  consts::ConstGenerator,
  identifier::IdentifierGenerator,
  method::MethodGenerator,
  property::PropertyGenerator,
  traituse::TraitUseGenerator,
};

pub struct ClassGenerator {}

impl ClassGenerator {
  pub fn generate(generator: &mut Generator, builder: &mut Builder, node: &Node) {
    let node = guard_ok!(node.to_owned().cast::<ClassNode>(), {
      return;
    });
    if node.modifier.len() > 0 {
      builder.push(format!("{} ", node.modifier).as_str());
    }
    builder.push("class ");
    IdentifierGenerator::generate(generator, builder, &node.name);
    if let Some(n) = &node.extends {
      builder.push(" extends ");
      IdentifierGenerator::generate(generator, builder, &n);
    }
    if node.implements.len() > 0 {
      builder.push(" implements ");
      let implements = generator.generate_nodes_new(
        &node.implements,
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
        (NodeType::TraitUse, TraitUseGenerator::generate),
        (NodeType::ConstProperty, ConstGenerator::generate_property),
        (NodeType::Property, PropertyGenerator::generate),
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
    test("abstract class A {\n}");
    test("final class A extends B {\n}");
    test("class A implements C, D, E {\n}");
  }
}
