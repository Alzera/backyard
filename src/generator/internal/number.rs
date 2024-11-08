use crate::{
  generator::generator::{ Builder, Generator, GeneratorArgument },
  guard_ok,
  parser::{ node::{ Node, NodeTraitCast }, nodes::number::NumberNode },
};

pub struct NumberGenerator {}

impl NumberGenerator {
  pub fn generate(
    _: &mut Generator,
    builder: &mut Builder,
    node: &Node,
    _: &mut GeneratorArgument
  ) {
    let node = guard_ok!(node.to_owned().cast::<NumberNode>(), {
      return;
    });
    builder.push(&node.value);
  }
}
