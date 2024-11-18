use backyard_nodes::{ cast_node, node::{ Node, NodeWrapper } };

use crate::generator::{ Builder, Generator };

use super::function::FunctionGenerator;

pub struct MethodGenerator {}

impl MethodGenerator {
  pub fn generate(generator: &mut Generator, builder: &mut Builder, node: &Box<Node>) {
    let node = cast_node!(NodeWrapper::Method, &node.node);
    if node.visibility.len() > 0 {
      builder.push(format!("{} ", node.visibility).as_str());
    }
    if node.modifier.len() > 0 {
      builder.push(format!("{} ", node.modifier).as_str());
    }
    if node.is_static {
      builder.push("static ");
    }
    FunctionGenerator::generate(generator, builder, &node.function);
  }
}

#[cfg(test)]
mod tests {
  use crate::test_utils::test;

  #[test]
  fn basic() {
    test(
      "abstract class A {
  function a(int $x, int $y = 0): int {
  }
  public final static function b(int $x, int $y = 0): int;
}"
    );
  }
}
