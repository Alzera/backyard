use backyard_nodes::{ cast_node, node::{ Node, NodeWrapper } };

use crate::generator::{ Builder, Generator, GeneratorArgument };

use super::block::BlockGenerator;

pub struct DoWhileGenerator;

impl DoWhileGenerator {
  pub fn generate(generator: &mut Generator, builder: &mut Builder, node: &Box<Node>) {
    let node = cast_node!(NodeWrapper::DoWhile, &node.node);

    builder.push("do");
    BlockGenerator::generate(generator, builder, &node.body, None);
    builder.push(" ");
    generator.generate_node(builder, &node.condition, &mut GeneratorArgument::default());
  }

  pub fn generate_condition(generator: &mut Generator, builder: &mut Builder, node: &Box<Node>) {
    let node = cast_node!(NodeWrapper::DoWhileCondition, &node.node);

    builder.push("while (");
    generator.generate_node(builder, &node.condition, &mut GeneratorArgument::default());
    builder.push(");");
  }
}

#[cfg(test)]
mod tests {
  use crate::test_utils::test_eval;

  #[test]
  fn basic() {
    test_eval("do {
} while (false);");
    test_eval("do {
} 
// this comment
while (false);");
  }
}
