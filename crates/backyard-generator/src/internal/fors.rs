use backyard_nodes::{ cast_node, node::{ BodyType, Node, NodeWrapper } };

use crate::generator::{ Builder, Generator, GeneratorArgument, DEFAULT_GENERATORS };

use super::block::BlockGenerator;

pub struct ForGenerator;

impl ForGenerator {
  pub fn generate(generator: &mut Generator, builder: &mut Builder, node: &Box<Node>) {
    let node = cast_node!(NodeWrapper::For, &node.node);

    builder.push("for (");
    let mut inits = Builder::new();
    if !node.inits.is_empty() {
      generator.generate_nodes(
        &mut inits,
        &node.inits,
        &mut GeneratorArgument::for_parameter(&DEFAULT_GENERATORS)
      );
    }
    builder.push(&inits.to_string(" "));
    builder.push("; ");
    let mut tests = Builder::new();
    if !node.tests.is_empty() {
      generator.generate_nodes(
        &mut tests,
        &node.tests,
        &mut GeneratorArgument::for_parameter(&DEFAULT_GENERATORS)
      );
    }
    builder.push(&tests.to_string(" "));
    builder.push("; ");
    let mut increments = Builder::new();
    if !node.increments.is_empty() {
      generator.generate_nodes(
        &mut increments,
        &node.increments,
        &mut GeneratorArgument::for_parameter(&DEFAULT_GENERATORS)
      );
    }
    builder.push(&increments.to_string(" "));
    builder.push(")");
    match node.body_type {
      BodyType::Basic => {
        if let Some(n) = &node.body {
          BlockGenerator::generate(generator, builder, n, None);
        }
      }
      BodyType::Short => {
        if let Some(n) = &node.body {
          BlockGenerator::generate(generator, builder, n, Some("endfor;"));
        }
      }
      BodyType::Empty => {
        builder.push(";");
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::test_utils::test_eval;

  #[test]
  fn basic() {
    test_eval("for (; ; ):\nendfor;");
    test_eval("for ($i = 1; $i <= 10; $i++) {\n}");
    test_eval("for ($i = 1; ; $i++) {\n}");
    test_eval("for ($i = 1, $j = 0; $i <= 10; $j += $i, print $i, $i++);");
  }
}
