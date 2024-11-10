use crate::{
  generator::generator::{ Builder, Generator, GeneratorArgument },
  guard,
  parser::{ node::{ Node, NodeTraitCast }, nodes::pre::PreNode },
};

pub struct PreGenerator {}

impl PreGenerator {
  pub fn generate(generator: &mut Generator, builder: &mut Builder, node: &Node) {
    let node = guard!(node.to_owned().cast::<PreNode>());
    builder.push(&node.operator.as_str());
    generator.generate_node(builder, &node.variable, &mut GeneratorArgument::default());
  }
}

#[cfg(test)]
mod tests {
  use crate::test_utils::test;

  #[test]
  fn basic() {
    test("$a = ++($a++);");
    test("~$a;");
    test("!$a;");
  }
}
