use crate::{
  generator::generator::{ Builder, Generator, GeneratorArgument },
  guard,
  parser::{ node::{ Node, NodeTraitCast }, nodes::dowhile::DoWhileNode },
};

use super::block::BlockGenerator;

pub struct DoWhileGenerator {}

impl DoWhileGenerator {
  pub fn generate(generator: &mut Generator, builder: &mut Builder, node: &Node) {
    let node = guard!(node.to_owned().cast::<DoWhileNode>());

    builder.push("do");
    BlockGenerator::generate(generator, builder, &node.body, None);
    builder.push(" while (");
    generator.generate_node(builder, &node.condition, &mut GeneratorArgument::default());
    builder.push(");");
  }
}

#[cfg(test)]
mod tests {
  use crate::test_utils::test;

  #[test]
  fn basic() {
    test("do {\n} while (false);");
  }
}
