use crate::{
  generator::generator::{ Builder, Generator, GeneratorArgument },
  guard_ok,
  parser::{ node::{ Node, NodeTraitCast }, nodes::consts::{ ConstNode, ConstPropertyNode } },
};

pub struct ConstGenerator {}

impl ConstGenerator {
  pub fn generate(generator: &mut Generator, builder: &mut Builder, node: &Node) {
    let node = guard_ok!(node.to_owned().cast::<ConstNode>(), {
      return;
    });

    builder.push("const ");
    let mut consts = generator.generate_nodes_new(&node.consts, &mut GeneratorArgument::default());
    if
      Generator::check_nodes_has_comments(&node.consts) ||
      2 + builder.last_len() + consts.total_len_with_separator(", ") > generator.max_length
    {
      consts.indent();
      builder.extend_first_line(&consts);
    } else {
      builder.push(&consts.to_string(" "));
    }
  }

  pub fn generate_property(generator: &mut Generator, builder: &mut Builder, node: &Node) {
    let node = guard_ok!(node.to_owned().cast::<ConstPropertyNode>(), {
      return;
    });

    if node.visibility.len() > 0 {
      builder.push(format!("{} ", node.visibility).as_str());
    }
    builder.push("const ");
    let mut consts = generator.generate_nodes_new(&node.consts, &mut GeneratorArgument::default());
    if
      Generator::check_nodes_has_comments(&node.consts) ||
      2 + builder.last_len() + consts.total_len_with_separator(", ") > generator.max_length
    {
      consts.indent();
      builder.extend_first_line(&consts);
    } else {
      builder.push(&consts.to_string(" "));
    }
  }
}
