use backyard_nodes::{ cast_node, node::{ Node, NodeWrapper } };

use crate::generator::{ Builder, Generator };

pub struct TypeGenerator {}

impl TypeGenerator {
  pub fn generate(_: &mut Generator, builder: &mut Builder, node: &Box<Node>) {
    let node = cast_node!(NodeWrapper::Type, &node.node);
    if node.is_nullable {
      builder.push("?");
    }
    builder.push(&node.name.join("|"));
  }
}

#[cfg(test)]
mod tests {
  use crate::test_utils::test;

  #[test]
  fn basic() {
    test("class A {
  public int $instance = 4;
  public static ?A $instance = 4;
}");
  }
}
