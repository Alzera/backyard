use crate::{
  generator::generator::{ Builder, Generator, GeneratorArgument },
  guard,
  parser::{ node::{ Node, NodeTraitCast }, nodes::parenthesis::{ CastNode, ParenthesisNode } },
};

pub struct ParenthesisGenerator {}

impl ParenthesisGenerator {
  pub fn generate(generator: &mut Generator, builder: &mut Builder, node: &Node) {
    let node = guard!(node.to_owned().cast::<ParenthesisNode>());
    builder.push("(");
    generator.generate_node(builder, &node.statement, &mut GeneratorArgument::default());
    builder.push(")");
  }

  pub fn generate_cast(generator: &mut Generator, builder: &mut Builder, node: &Node) {
    let node = guard!(node.to_owned().cast::<CastNode>());
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
