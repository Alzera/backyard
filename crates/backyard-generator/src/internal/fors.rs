use backyard_nodes::{ cast_node, BodyType, Node, NodeWrapper };

use crate::generator::{ Builder, Generator, GeneratorArgument, DEFAULT_GENERATORS };

use super::block::BlockGenerator;

pub struct ForGenerator;

impl ForGenerator {
  pub fn generate<'arena>(
    generator: &mut Generator<'arena, '_>,
    builder: &mut Builder,
    node: &Node<'arena>
  ) {
    let node = cast_node!(For, &node.wrapper);

    builder.push("for (");
    let mut inits = Builder::new();
    if !node.inits.is_empty() {
      generator.generate_nodes(
        &mut inits,
        &node.inits,
        &mut GeneratorArgument::for_parameter(&DEFAULT_GENERATORS)
      );
    }
    builder.push(&inits.print(" "));
    builder.push("; ");
    let mut tests = Builder::new();
    if !node.tests.is_empty() {
      generator.generate_nodes(
        &mut tests,
        &node.tests,
        &mut GeneratorArgument::for_parameter(&DEFAULT_GENERATORS)
      );
    }
    builder.push(&tests.print(" "));
    builder.push("; ");
    let mut increments = Builder::new();
    if !node.increments.is_empty() {
      generator.generate_nodes(
        &mut increments,
        &node.increments,
        &mut GeneratorArgument::for_parameter(&DEFAULT_GENERATORS)
      );
    }
    builder.push(&increments.print(" "));
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
