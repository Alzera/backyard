use crate::{
  generator::generator::{ Builder, Generator },
  guard_ok,
  parser::{ node::{ Node, NodeTraitCast }, nodes::method::MethodNode },
};

use super::function::FunctionGenerator;

pub struct MethodGenerator {}

impl MethodGenerator {
  pub fn generate(generator: &mut Generator, builder: &mut Builder, node: &Node) {
    let node = guard_ok!(node.to_owned().cast::<MethodNode>(), {
      return;
    });
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
  public function b(int $x, int $y = 0): int;
}"
    );
  }
}
