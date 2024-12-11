use backyard_nodes::{ cast_node, node::{ Node, NodeWrapper } };

use crate::generator::{ Builder, Generator, GeneratorArgument, DEFAULT_GENERATORS };

pub struct ConstGenerator;

impl ConstGenerator {
  pub fn generate<'arena>(
    generator: &mut Generator<'arena, '_>,
    builder: &mut Builder,
    node: &Node<'arena>
  ) {
    let node = cast_node!(Const, &node.node);

    builder.push("const ");
    let mut consts = generator.generate_nodes_new(
      &node.items,
      &mut GeneratorArgument::for_parameter(&DEFAULT_GENERATORS)
    );
    if
      Generator::check_nodes_has_comments(&node.items) ||
      2 + builder.last_len() + consts.total_len_with_separator(" ") > generator.max_length
    {
      consts.indent();
      builder.extend_first_line(consts);
    } else {
      builder.push(&consts.print(" "));
    }
  }

  pub fn generate_property<'arena>(
    generator: &mut Generator<'arena, '_>,
    builder: &mut Builder,
    node: &Node<'arena>
  ) {
    let node = cast_node!(ConstProperty, &node.node);

    for visibility in &node.visibilities {
      builder.push(&format!("{} ", visibility));
    }
    builder.push("const ");
    if let Some(const_type) = &node.const_type {
      generator.generate_node(builder, const_type, &mut GeneratorArgument::default());
      builder.push(" ");
    }
    let mut consts = generator.generate_nodes_new(
      &node.items,
      &mut GeneratorArgument::for_parameter(&DEFAULT_GENERATORS)
    );
    if
      Generator::check_nodes_has_comments(&node.items) ||
      2 + builder.last_len() + consts.total_len_with_separator(" ") > generator.max_length
    {
      consts.indent();
      builder.extend_first_line(consts);
    } else {
      builder.push(&consts.print(" "));
    }
  }
}
