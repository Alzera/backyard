use backyard_nodes::{ cast_node, node::{ BodyType, Node, NodeType, NodeWrapper } };

use crate::generator::{ Builder, Generator, GeneratorArgument };

use super::{ block::BlockGenerator, identifier::IdentifierGenerator };

pub struct DeclareGenerator;

impl DeclareGenerator {
  pub fn generate<'arena, 'a>(
    generator: &mut Generator<'arena, 'a>,
    builder: &mut Builder,
    node: &Node<'arena>
  ) {
    let node = cast_node!(Declare, &node.node);
    builder.push("declare");
    let mut arguments = generator.generate_nodes_new(
      &node.arguments,
      &mut GeneratorArgument::for_parameter(&[(NodeType::DeclareArgument, Self::generate_argument)])
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
    match node.body_type {
      BodyType::Basic => {
        if let Some(n) = &node.body {
          BlockGenerator::generate(generator, builder, n, None);
        }
      }
      BodyType::Short => {
        if let Some(n) = &node.body {
          BlockGenerator::generate(generator, builder, n, Some("enddeclare;"));
        }
      }
      BodyType::Empty => {
        builder.push(";");
      }
    }
  }

  pub fn generate_argument<'arena, 'a>(
    generator: &mut Generator<'arena, 'a>,
    builder: &mut Builder,
    node: &Node<'arena>
  ) {
    let node = cast_node!(DeclareArgument, &node.node);
    IdentifierGenerator::generate(generator, builder, &node.name);
    builder.push(" = ");
    generator.generate_node(builder, &node.value, &mut GeneratorArgument::default());
  }
}
