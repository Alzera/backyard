use backyard_nodes::{ cast_node, node::{ Node, NodeType, NodeWrapper } };

use crate::generator::{ Builder, EndMode, Generator, GeneratorArgument };

use super::identifier::IdentifierGenerator;

pub struct EnumGenerator {}

impl EnumGenerator {
  pub fn generate(generator: &mut Generator, builder: &mut Builder, node: &Box<Node>) {
    let node = cast_node!(NodeWrapper::Enum, &node.node);
    builder.push("enum ");
    IdentifierGenerator::generate(generator, builder, &node.name);
    let mut items = generator.generate_nodes_new(
      &node.items,
      &mut GeneratorArgument::new(
        EndMode::SemicolonDynamic,
        &[(NodeType::EnumItem, Self::generate_item)]
      )
    );
    builder.push(" {");
    items.indent();
    builder.extend(&items);
    builder.new_line();
    builder.push("}");
  }

  pub fn generate_item(generator: &mut Generator, builder: &mut Builder, node: &Box<Node>) {
    let node = cast_node!(NodeWrapper::EnumItem, &node.node);
    builder.push("case ");
    generator.generate_node(builder, &node.value, &mut GeneratorArgument::default());
  }
}

#[cfg(test)]
mod tests {
  use crate::test_utils::test;

  #[test]
  fn basic() {
    test("enum Suit {\n  case Hearts;\n  case Spades;\n}");
  }
}
