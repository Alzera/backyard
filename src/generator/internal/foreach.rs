use crate::{
  generator::generator::{ Builder, Generator, GeneratorArgument },
  guard,
  parser::{ node::{ Node, NodeTraitCast }, nodes::foreach::ForeachNode },
};

use super::block::BlockGenerator;

pub struct ForeachGenerator {}

impl ForeachGenerator {
  pub fn generate(generator: &mut Generator, builder: &mut Builder, node: &Node) {
    let node = guard!(node.to_owned().cast::<ForeachNode>());

    builder.push("foreach (");
    generator.generate_node(builder, &node.source, &mut GeneratorArgument::default());
    builder.push(" as ");
    if let Some(key) = &node.key {
      generator.generate_node(builder, key, &mut GeneratorArgument::default());
      builder.push(" => ");
    }
    generator.generate_node(builder, &node.value, &mut GeneratorArgument::default());
    builder.push(")");
    let end = if node.is_short { Some("endforeach;") } else { None };
    BlockGenerator::generate(generator, builder, &node.body, end);
  }
}

#[cfg(test)]
mod tests {
  use crate::test_utils::test;

  #[test]
  fn basic() {
    test("foreach ($A as $x):\nendforeach;");
    test("foreach ($A as &$x) {\n}");
    test("foreach ($arr as $key => $value) {\n}");
  }
}
