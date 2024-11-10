use crate::{
  generator::generator::{ Builder, Generator },
  guard,
  parser::{ node::{ Node, NodeTraitCast }, nodes::number::NumberNode },
};

pub struct NumberGenerator {}

impl NumberGenerator {
  pub fn generate(_: &mut Generator, builder: &mut Builder, node: &Node) {
    let node = guard!(node.to_owned().cast::<NumberNode>());
    builder.push(&node.value);
  }
}

#[cfg(test)]
mod tests {
  use crate::test_utils::test;

  #[test]
  fn basic() {
    test("$a = 0 + 0.5 + .5 + 0x2e45;");
  }
}
