use crate::{
  generator::generator::{ Builder, Generator },
  guard_ok,
  parser::{ node::{ Node, NodeTraitCast }, nodes::types::TypeNode },
};

pub struct TypeGenerator {}

impl TypeGenerator {
  pub fn generate(_: &mut Generator, builder: &mut Builder, node: &Node) {
    let node = guard_ok!(node.to_owned().cast::<TypeNode>(), {
      return;
    });
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
