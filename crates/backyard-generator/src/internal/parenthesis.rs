use backyard_nodes::{ cast_node, node::{ Node, NodeWrapper } };

use crate::generator::{ Builder, Generator, GeneratorArgument };

pub struct ParenthesisGenerator {}

impl ParenthesisGenerator {
  pub fn generate(generator: &mut Generator, builder: &mut Builder, node: &Box<Node>) {
    let node = cast_node!(NodeWrapper::Parenthesis, &node.node);
    builder.push("(");
    generator.generate_node(builder, &node.statement, &mut GeneratorArgument::default());
    builder.push(")");
  }

  pub fn generate_cast(generator: &mut Generator, builder: &mut Builder, node: &Box<Node>) {
    let node = cast_node!(NodeWrapper::Cast, &node.node);
    builder.push("(");
    generator.generate_node(builder, &node.target, &mut GeneratorArgument::default());
    builder.push(") ");
    generator.generate_node(builder, &node.expression, &mut GeneratorArgument::default());
  }
}

#[cfg(test)]
mod tests {
  use crate::test_utils::test;

  #[test]
  fn basic() {
    test("$a = (int) $a;");
    test("$a = 5 + 0.5 + (.5 + 0x2e45);");
    test("(fn () => 0)();");
  }
}