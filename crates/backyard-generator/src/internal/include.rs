use backyard_nodes::{ cast_node, node::{ Node, NodeWrapper } };

use crate::generator::{ Builder, Generator, GeneratorArgument };

pub struct IncludeGenerator {}

impl IncludeGenerator {
  pub fn generate(generator: &mut Generator, builder: &mut Builder, node: &Box<Node>) {
    let node = cast_node!(NodeWrapper::Include, &node.node);
    if node.is_require {
      builder.push("require");
    } else {
      builder.push("include");
    }
    if node.is_once {
      builder.push("_once");
    }
    builder.push("(");
    generator.generate_node(builder, &node.argument, &mut GeneratorArgument::default());
    builder.push(")");
  }
}

#[cfg(test)]
mod tests {
  use crate::test_utils::test;

  #[test]
  fn basic() {
    test("require(__DIR__ . \"/something_that_does_not_exist\");");
    test("require_once(__DIR__ . \"/something_that_does_not_exist\");");
    test("include(\"something_that_does_not_exist\");");
    test("include_once(\"something_that_does_not_exist\");");
  }
}
