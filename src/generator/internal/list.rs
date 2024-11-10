use crate::{
  generator::generator::{ Builder, Generator, GeneratorArgument, DEFAULT_GENERATORS },
  guard_ok,
  parser::{ node::{ Node, NodeTraitCast }, nodes::list::ListNode },
};

pub struct ListGenerator {}

impl ListGenerator {
  pub fn generate(generator: &mut Generator, builder: &mut Builder, node: &Node) {
    let node = guard_ok!(node.to_owned().cast::<ListNode>(), {
      return;
    });
    builder.push("list(");
    let mut values = generator.generate_nodes_new(
      &node.values,
      &mut GeneratorArgument::for_parameter(&DEFAULT_GENERATORS)
    );
    if
      Generator::check_nodes_has_comments(&node.values) ||
      2 + builder.last_len() + values.total_len_with_separator(" ") > generator.max_length
    {
      values.indent();
      builder.extend(&values);
      builder.new_line();
    } else {
      builder.push(&values.to_string(" "));
    }
    builder.push(")");
  }
}

#[cfg(test)]
mod tests {
  use crate::test_utils::test;

  #[test]
  fn basic() {
    test("list($a, $b) = [0, 1];");
  }
}
