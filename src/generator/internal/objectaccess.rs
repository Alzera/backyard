use crate::{
  generator::generator::{ Builder, Generator, GeneratorArgument },
  guard_ok,
  parser::{ node::{ Node, NodeTraitCast, NodeType }, nodes::objectaccess::ObjectAccessNode },
};

use super::identifier::IdentifierGenerator;

pub struct ObjectAccessGenerator {}

impl ObjectAccessGenerator {
  pub fn generate(generator: &mut Generator, builder: &mut Builder, node: &Node) {
    let node = guard_ok!(node.to_owned().cast::<ObjectAccessNode>(), {
      return;
    });
    generator.generate_node(builder, &node.object, &mut GeneratorArgument::default());
    builder.push("->");
    if node.property.get_type() == NodeType::Identifier {
      IdentifierGenerator::generate(generator, builder, &node.property);
    } else {
      builder.push("{");
      generator.generate_node(builder, &node.property, &mut GeneratorArgument::default());
      builder.push("}");
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::test_utils::test;

  #[test]
  fn basic() {
    test("$a->b;");
    test("$a->{\"b\"};");
  }
}
