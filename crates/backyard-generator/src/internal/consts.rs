use backyard_nodes::{ cast_node, node::{ Node, NodeWrapper } };

use crate::generator::{ Builder, Generator, GeneratorArgument, DEFAULT_GENERATORS };

pub struct ConstGenerator;

impl ConstGenerator {
  pub fn generate(generator: &mut Generator, builder: &mut Builder, node: &Box<Node>) {
    let node = cast_node!(NodeWrapper::Const, &node.node);

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
      builder.extend_first_line(&consts);
    } else {
      builder.push(&consts.to_string(" "));
    }
  }

  pub fn generate_property(generator: &mut Generator, builder: &mut Builder, node: &Box<Node>) {
    let node = cast_node!(NodeWrapper::ConstProperty, &node.node);

    if !node.visibility.is_empty() {
      builder.push(format!("{} ", node.visibility).as_str());
    }
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
      builder.extend_first_line(&consts);
    } else {
      builder.push(&consts.to_string(" "));
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::test_utils::test_eval;

  #[test]
  fn basic() {
    test_eval("class A {\n  const A = 0, B = 1;\n}");
    test_eval("class A {\n  public const A = 0, B = 1;\n}");
    test_eval("const A = 0, B = 1;");
  }
}
