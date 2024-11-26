use backyard_nodes::{ cast_node, node::{ Node, NodeWrapper } };

use crate::generator::{ Builder, Generator };

pub struct MagicGenerator;

impl MagicGenerator {
  pub fn generate(_: &mut Generator, builder: &mut Builder, node: &Box<Node>) {
    let node = cast_node!(NodeWrapper::Magic, &node.node);
    builder.push(&node.name)
  }
}

#[cfg(test)]
mod tests {
  use crate::test_utils::test_eval;

  #[test]
  fn basic() {
    test_eval("__DIR__ . \"/something_that_does_not_exist\";");
  }
}
