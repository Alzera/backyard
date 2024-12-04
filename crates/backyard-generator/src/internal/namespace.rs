use backyard_nodes::{ cast_node, node::{ Node, NodeWrapper } };

use crate::generator::{ Builder, Generator, DEFAULT_GENERATORS };

use super::block::BlockGenerator;

pub struct NamespaceGenerator;

impl NamespaceGenerator {
  pub fn generate(generator: &mut Generator, builder: &mut Builder, node: &Box<Node>) {
    let node = cast_node!(NodeWrapper::Namespace, &node.node);
    builder.push("namespace ");
    builder.push(&node.name);
    if node.is_bracket {
      BlockGenerator::generate(generator, builder, &node.body, None);
    } else {
      builder.push(";");
      let body = BlockGenerator::generate_base(generator, &node.body, &DEFAULT_GENERATORS);
      builder.extend(body);
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::test_utils::test_eval;

  #[test]
  fn basic() {
    test_eval("namespace MyApp\\A {\n}");
    test_eval("namespace MyApp\\B;");
  }
}
