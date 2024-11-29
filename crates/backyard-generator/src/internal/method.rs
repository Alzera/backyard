use backyard_nodes::{ cast_node, node::{ Node, NodeWrapper } };

use crate::generator::{ Builder, Generator };

use super::function::FunctionGenerator;

pub struct MethodGenerator;

impl MethodGenerator {
  pub fn generate(generator: &mut Generator, builder: &mut Builder, node: &Box<Node>) {
    let node = cast_node!(NodeWrapper::Method, &node.node);
    if let Some(n) = &node.visibility {
      builder.push(format!("{} ", n).as_str());
    }
    if let Some(n) = &node.inheritance {
      builder.push(format!("{} ", n).as_str());
    }
    if node.is_static {
      builder.push("static ");
    }
    FunctionGenerator::generate(generator, builder, &node.function);
  }
}

#[cfg(test)]
mod tests {
  use crate::test_utils::test_eval;

  #[test]
  fn basic() {
    test_eval(
      "abstract class A {
  public function dayOfYear(?array &$matches, array|int &$matches = null): static|int {
  }
  public final static function b(int $x, int $y = 0): int;
}"
    );
  }
}
