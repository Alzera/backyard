use crate::{
  generator::generator::{ Builder, Generator, GeneratorArgument },
  guard_ok,
  parser::{ node::{ Node, NodeTraitCast }, nodes::program::ProgramNode },
};

pub struct ProgramGenerator {}

impl ProgramGenerator {
  pub fn generate(
    generator: &mut Generator,
    builder: &mut Builder,
    node: &Node,
    args: &mut GeneratorArgument
  ) {
    let node = guard_ok!(node.to_owned().cast::<ProgramNode>(), {
      return;
    });
    builder.push("<?php");
    builder.new_line();
    generator.generate_nodes(builder, &node.children, args);
  }
}
