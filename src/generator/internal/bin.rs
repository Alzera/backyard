use crate::{
  generator::generator::{ Builder, Generator, GeneratorArgument },
  guard_ok,
  parser::{ node::{ Node, NodeTraitCast }, nodes::bin::BinNode },
};

pub struct BinGenerator {}

impl BinGenerator {
  pub fn generate(
    generator: &mut Generator,
    builder: &mut Builder,
    node: &Node,
    args: &mut GeneratorArgument
  ) {
    let node = guard_ok!(node.to_owned().cast::<BinNode>(), {
      return;
    });
    generator.generate_node(builder, &node.left, args);
    let mut expr = generator.generate_node_new(&node.right, args);
    if builder.last_len() + expr.first_len() + node.operator.len() > args.max_length {
      expr.indent();
      builder.new_line();
      builder.push(format!("{} ", node.operator).as_str());
      builder.indent_last();
      builder.extend_first_line(&expr);
    } else {
      builder.push(format!(" {} ", node.operator).as_str());
      builder.extend_first_line(&expr);
    }
  }
}
