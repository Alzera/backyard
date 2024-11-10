use crate::{
  generator::generator::{ Builder, Generator, GeneratorArgument },
  guard,
  parser::{ node::{ Node, NodeTraitCast }, nodes::post::PostNode },
};

pub struct PostGenerator {}

impl PostGenerator {
  pub fn generate(generator: &mut Generator, builder: &mut Builder, node: &Node) {
    let node = guard!(node.to_owned().cast::<PostNode>());
    generator.generate_node(builder, &node.variable, &mut GeneratorArgument::default());
    builder.push(&node.operator.as_str());
  }
}

#[cfg(test)]
mod tests {
  use crate::test_utils::test;

  #[test]
  fn basic() {
    test("$a = ++($a++);");
    test("$a = --($a--);");
  }
}
