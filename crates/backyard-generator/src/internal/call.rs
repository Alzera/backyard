use backyard_nodes::{ cast_node, node::{ Node, NodeType, NodeWrapper } };

use crate::generator::{ Builder, Generator, GeneratorArgument };

pub struct CallGenerator;

impl CallGenerator {
  pub fn generate<'arena>(
    generator: &mut Generator<'arena, '_>,
    builder: &mut Builder,
    node: &Node<'arena>
  ) {
    let node = cast_node!(Call, &node.node);
    generator.generate_node(builder, &node.name, &mut GeneratorArgument::default());
    let mut arguments = generator.generate_nodes_new(
      &node.arguments,
      &mut GeneratorArgument::for_parameter(&[(NodeType::CallArgument, Self::generate_argument)])
    );
    builder.push("(");
    if
      Generator::check_nodes_has_comments(&node.arguments) ||
      2 + builder.last_len() + arguments.total_len_with_separator(" ") > generator.max_length
    {
      arguments.indent();
      builder.extend(arguments);
      builder.new_line();
    } else {
      builder.push(&arguments.print(" "));
    }
    builder.push(")");
  }

  pub fn generate_argument<'arena>(
    generator: &mut Generator<'arena, '_>,
    builder: &mut Builder,
    node: &Node<'arena>
  ) {
    let node = cast_node!(CallArgument, &node.node);
    if let Some(name) = &node.name {
      generator.generate_node(builder, name, &mut GeneratorArgument::default());
      builder.push(": ");
    }
    generator.generate_node(builder, &node.value, &mut GeneratorArgument::default());
  }
}
