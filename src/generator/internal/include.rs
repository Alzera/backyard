use crate::{
  generator::generator::{ Builder, Generator, GeneratorArgument },
  guard_ok,
  parser::{ node::{ Node, NodeTraitCast }, nodes::include::IncludeNode },
};

pub struct IncludeGenerator {}

impl IncludeGenerator {
  pub fn generate(generator: &mut Generator, builder: &mut Builder, node: &Node) {
    let node = guard_ok!(node.to_owned().cast::<IncludeNode>(), {
      return;
    });
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
