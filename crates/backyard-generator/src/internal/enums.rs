use backyard_nodes::{ cast_node, node::{ Node, NodeType, NodeWrapper } };

use crate::generator::{ Builder, EndMode, Generator, GeneratorArgument };

use super::{ consts::ConstGenerator, identifier::IdentifierGenerator, method::MethodGenerator };

pub struct EnumGenerator {}

impl EnumGenerator {
  pub fn generate(generator: &mut Generator, builder: &mut Builder, node: &Box<Node>) {
    let node = cast_node!(NodeWrapper::Enum, &node.node);
    builder.push("enum ");
    IdentifierGenerator::generate(generator, builder, &node.name);
    if let Some(n) = &node.enum_type {
      builder.push(": ");
      IdentifierGenerator::generate(generator, builder, &n);
    }
    if let Some(n) = &node.implements {
      builder.push(" implements ");
      IdentifierGenerator::generate(generator, builder, &n);
    }
    let mut items = generator.generate_nodes_new(
      &node.items,
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
  use crate::test_utils::test_eval;

  #[test]
  fn basic() {
    test_eval("enum Suit {
  case Hearts;
  case Spades;
}");
    test_eval("enum Suit: int {
  case Hearts;
  case Spades;
}");
    test_eval(
      "enum Suit implements SuitInterface {
  case Hearts;
  case Spades;
  public const MY_CONST = \"constant\";
  public function color(): string {
    return match($this) {
      Suit::Hearts, Suit::Diamonds => 'Red',
      Suit::Clubs, Suit::Spades => 'Black'
    };
  }
}"
    );
  }
}
