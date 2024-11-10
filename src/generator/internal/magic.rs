use crate::{
  generator::generator::{ Builder, Generator },
  guard_ok,
  parser::{ node::{ Node, NodeTraitCast }, nodes::magic::MagicNode },
};

pub struct MagicGenerator {}

impl MagicGenerator {
  pub fn generate(_: &mut Generator, builder: &mut Builder, node: &Node) {
    let node = guard_ok!(node.to_owned().cast::<MagicNode>(), {
      return;
    });
    builder.push(&node.name)
  }
}

#[cfg(test)]
mod tests {
  use crate::test_utils::test;

  #[test]
  fn basic() {
    test("__DIR__ . \"/something_that_does_not_exist\";");
  }
}
