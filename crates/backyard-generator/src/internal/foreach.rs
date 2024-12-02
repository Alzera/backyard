use backyard_nodes::{ cast_node, node::{ Node, NodeType, NodeWrapper } };

use crate::generator::{ Builder, Generator, GeneratorArgument };

use super::block::BlockGenerator;

pub struct ForeachGenerator;

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
    if node.is_short {
      let end = if node.is_short { Some("endforeach;") } else { None };
      BlockGenerator::generate(generator, builder, &node.body, end);
    } else {
      if node.body.node_type == NodeType::Block {
        BlockGenerator::generate(generator, builder, &node.body, None);
      } else {
        builder.push(" ");
        generator.generate_node(builder, &node.body, &mut GeneratorArgument::for_block());
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::test_utils::test_eval;

  #[test]
  fn basic() {
    test_eval("foreach ($escape as $probe) $pattern = 5;");
    test_eval("foreach ($A as $x):
endforeach;");
    test_eval("foreach ($data as $k => $value):
endforeach;");
    test_eval("foreach ($A as &$x) {\n}");
    test_eval("foreach ($arr as $key => $value) {\n}");
  }
}
