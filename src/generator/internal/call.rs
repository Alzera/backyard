use crate::{
  generator::generator::{ Builder, Generator, GeneratorArgument },
  guard_ok,
  parser::{ node::{ Node, NodeTraitCast, NodeType }, nodes::call::{ ArgumentNode, CallNode } },
};

pub struct CallGenerator {}

impl CallGenerator {
  pub fn generate(generator: &mut Generator, builder: &mut Builder, node: &Node) {
    let node = guard_ok!(node.to_owned().cast::<CallNode>(), {
      return;
    });
    generator.generate_node(builder, &node.name, &mut GeneratorArgument::default());
    let mut arguments = generator.generate_nodes_new(
      &node.arguments,
      &mut GeneratorArgument::for_parameter(&[(NodeType::Argument, Self::generate_argument)])
    );
    builder.push("(");
    if
      Generator::check_nodes_has_comments(&node.arguments) ||
      2 + builder.last_len() + arguments.total_len_with_separator(", ") > generator.max_length
    {
      arguments.indent();
      builder.extend(&arguments);
      builder.new_line();
    } else {
      builder.push(&arguments.to_string(" "));
    }
    builder.push(")");
  }

  pub fn generate_argument(generator: &mut Generator, builder: &mut Builder, node: &Node) {
    let node = guard_ok!(node.to_owned().cast::<ArgumentNode>(), {
      return;
    });
    if let Some(name) = &node.name {
      generator.generate_node(builder, name, &mut GeneratorArgument::default());
      builder.push(": ");
    }
    generator.generate_node(builder, &node.value, &mut GeneratorArgument::default());
  }
}
