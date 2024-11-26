use backyard_nodes::{ cast_node, node::{ Node, NodeWrapper } };

use crate::generator::{ Builder, Generator };

pub struct NumberGenerator;

impl NumberGenerator {
  pub fn generate(_: &mut Generator, builder: &mut Builder, node: &Box<Node>) {
    let node = cast_node!(NodeWrapper::Number, &node.node);
    builder.push(&node.value);
  }
}

#[cfg(test)]
mod tests {
  use crate::test_utils::test_eval;

  #[test]
  fn basic() {
    test_eval("$a = 0 + 0.5 + .5 + 0x2e45;");
  }
}
