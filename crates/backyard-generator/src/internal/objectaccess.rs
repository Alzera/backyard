use backyard_nodes::{ cast_node, node::{ Node, NodeType, NodeWrapper } };

use crate::generator::{ Builder, Generator, GeneratorArgument };

use super::identifier::IdentifierGenerator;

pub struct ObjectAccessGenerator {}

impl ObjectAccessGenerator {
  pub fn generate(generator: &mut Generator, builder: &mut Builder, node: &Box<Node>) {
    let node = cast_node!(NodeWrapper::ObjectAccess, &node.node);
    generator.generate_node(builder, &node.object, &mut GeneratorArgument::default());
    builder.push("->");
    if node.property.node_type == NodeType::Identifier {
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
