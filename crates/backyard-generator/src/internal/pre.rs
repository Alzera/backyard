use backyard_nodes::{ cast_node, node::{ Node, NodeWrapper } };

use crate::generator::{ Builder, Generator, GeneratorArgument };

pub struct PreGenerator {}

impl PreGenerator {
  pub fn generate(generator: &mut Generator, builder: &mut Builder, node: &Box<Node>) {
    let node = cast_node!(NodeWrapper::Pre, &node.node);
    builder.push(&node.operator.as_str());
    generator.generate_node(builder, &node.variable, &mut GeneratorArgument::default());
  }

  pub fn generate_negate(generator: &mut Generator, builder: &mut Builder, node: &Box<Node>) {
    let node = cast_node!(NodeWrapper::Negate, &node.node);
    builder.push("!");
    generator.generate_node(builder, &node.variable, &mut GeneratorArgument::default());
  }

  pub fn generate_silent(generator: &mut Generator, builder: &mut Builder, node: &Box<Node>) {
    let node = cast_node!(NodeWrapper::Silent, &node.node);
    builder.push("@");
    generator.generate_node(builder, &node.variable, &mut GeneratorArgument::default());
  }
}

#[cfg(test)]
mod tests {
  use crate::test_utils::test;

  #[test]
  fn basic() {
    test("$a = ++($a++);");
    test("!$a;");
    test("@$a;");
  }
}
