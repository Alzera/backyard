use crate::{
  generator::generator::{ Builder, Generator, GeneratorArgument },
  guard_ok,
  parser::{ node::{ Node, NodeTraitCast }, nodes::include::IncludeNode },
};

pub struct IncludeGenerator {}

impl IncludeGenerator {
  pub fn generate(generator: &mut Generator, builder: &mut Builder, node: &Node) {
    let node = guard_ok!(node.to_owned().cast::<IncludeNode>(), {
      return;
    });
    if node.is_require {
      builder.push("require");
    } else {
      builder.push("include");
    }
    if node.is_once {
      builder.push("_once");
    }
    builder.push(" ");
    generator.generate_node(builder, &node.argument, &mut GeneratorArgument::default());
  }
}
