use crate::{
  generator::generator::{ Builder, Generator, GeneratorArgument },
  guard,
  parser::{ node::{ Node, NodeTraitCast }, nodes::whiles::WhileNode },
};

use super::block::BlockGenerator;

pub struct WhileGenerator {}

impl WhileGenerator {
  pub fn generate(generator: &mut Generator, builder: &mut Builder, node: &Node) {
    let node = guard!(node.to_owned().cast::<WhileNode>());

    builder.push("while (");
    generator.generate_node(builder, &node.condition, &mut GeneratorArgument::default());
    builder.push(")");
    let end = if node.is_short { Some("endwhile;") } else { None };
    BlockGenerator::generate(generator, builder, &node.body, end);
  }
}

#[cfg(test)]
mod tests {
  use crate::test_utils::test;

  #[test]
  fn basic() {
    test("while ($i <= 10) {\n}");
    test("while ($i <= 10):\nendwhile;");
  }
}
