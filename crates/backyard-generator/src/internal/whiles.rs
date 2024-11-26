use backyard_nodes::{ cast_node, node::{ Node, NodeWrapper } };

use crate::generator::{ Builder, Generator, GeneratorArgument };

use super::block::BlockGenerator;

pub struct WhileGenerator;

impl WhileGenerator {
  pub fn generate(generator: &mut Generator, builder: &mut Builder, node: &Box<Node>) {
    let node = cast_node!(NodeWrapper::While, &node.node);

    builder.push("while (");
    generator.generate_node(builder, &node.condition, &mut GeneratorArgument::default());
    builder.push(")");
    let end = if node.is_short { Some("endwhile;") } else { None };
    BlockGenerator::generate(generator, builder, &node.body, end);
  }
}

#[cfg(test)]
mod tests {
  use crate::test_utils::test_eval;

  #[test]
  fn basic() {
    test_eval("while ($i <= 10) {\n}");
    test_eval("while ($i <= 10):\nendwhile;");
  }
}
