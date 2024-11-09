use crate::{
  generator::generator::{ Builder, Generator, GeneratorArgument, DEFAULT_GENERATORS },
  guard_ok,
  parser::{ node::{ BodyType, Node, NodeTraitCast }, nodes::fors::ForNode },
};

use super::block::BlockGenerator;

pub struct ForGenerator {}

impl ForGenerator {
  pub fn generate(generator: &mut Generator, builder: &mut Builder, node: &Node) {
    let node = guard_ok!(node.to_owned().cast::<ForNode>(), {
      return;
    });

    builder.push("for (");
    let mut inits = Builder::new();
    if node.inits.len() > 0 {
      generator.generate_nodes(
        &mut inits,
        &node.inits,
        &mut GeneratorArgument::for_parameter(&DEFAULT_GENERATORS)
      );
    }
    builder.push(&inits.to_string(" "));
    builder.push("; ");
    let mut tests = Builder::new();
    if node.tests.len() > 0 {
      generator.generate_nodes(
        &mut tests,
        &node.tests,
        &mut GeneratorArgument::for_parameter(&DEFAULT_GENERATORS)
      );
    }
    builder.push(&tests.to_string(" "));
    builder.push("; ");
    let mut increments = Builder::new();
    if node.increments.len() > 0 {
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
          BlockGenerator::generate(generator, builder, &n, None);
        }
      }
      BodyType::Short => {
        if let Some(n) = &node.body {
          BlockGenerator::generate(generator, builder, &n, Some("endfor;"));
        }
      }
      BodyType::Empty => {
        builder.push(";");
      }
    }
  }
}
