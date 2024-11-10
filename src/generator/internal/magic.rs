use crate::{
  generator::generator::{ Builder, Generator },
  guard,
  parser::{ node::{ Node, NodeTraitCast }, nodes::magic::MagicNode },
};

pub struct MagicGenerator {}

impl MagicGenerator {
  pub fn generate(_: &mut Generator, builder: &mut Builder, node: &Node) {
    let node = guard!(node.to_owned().cast::<MagicNode>());
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
