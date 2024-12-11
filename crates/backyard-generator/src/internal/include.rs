use backyard_nodes::{ cast_node, node::{ Node, NodeWrapper } };

use crate::generator::{ Builder, Generator, GeneratorArgument };

pub struct IncludeGenerator;

impl IncludeGenerator {
  pub fn generate<'arena>(
    generator: &mut Generator<'arena, '_>,
    builder: &mut Builder,
    node: &Node<'arena>
  ) {
    let node = cast_node!(Include, &node.wrapper);
    if node.is_require {
      builder.push("require");
    } else {
      builder.push("include");
    }
    if node.is_once {
      builder.push("_once");
    }
    if node.use_parenthesis {
      builder.push("(");
      generator.generate_node(builder, &node.argument, &mut GeneratorArgument::default());
      builder.push(")");
    } else {
      builder.push(" ");
      generator.generate_node(builder, &node.argument, &mut GeneratorArgument::default());
    }
  }
}
