use backyard_nodes::{ cast_node, node::{ Node, NodeWrapper } };

use crate::generator::{ Builder, Generator, GeneratorArgument };

use super::block::BlockGenerator;

pub struct ForeachGenerator {}

impl ForeachGenerator {
  pub fn generate(generator: &mut Generator, builder: &mut Builder, node: &Box<Node>) {
    let node = cast_node!(NodeWrapper::Foreach, &node.node);

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
  use crate::test_utils::test_eval;

  #[test]
  fn basic() {
    test_eval("foreach ($A as $x):\nendforeach;");
    test_eval("foreach ($A as &$x) {\n}");
    test_eval("foreach ($arr as $key => $value) {\n}");
  }
}
