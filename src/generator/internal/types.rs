use crate::{
  generator::generator::{ Builder, Generator, GeneratorArgument },
  guard_ok,
  parser::{ node::{ Node, NodeTraitCast }, nodes::types::TypeNode },
};

pub struct TypeGenerator {}

impl TypeGenerator {
  pub fn generate(
    _: &mut Generator,
    builder: &mut Builder,
    node: &Node,
    _: &mut GeneratorArgument
  ) {
    let node = guard_ok!(node.to_owned().cast::<TypeNode>(), {
      return;
    });
    if node.is_nullable {
      builder.push("?");
    }
    builder.push(&node.name.join("|"));
  }
}
