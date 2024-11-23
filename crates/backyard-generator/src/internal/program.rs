use backyard_nodes::{ cast_node, node::{ Node, NodeWrapper } };

use crate::generator::{ Builder, Generator, GeneratorArgument };

pub struct ProgramGenerator {}

impl ProgramGenerator {
  pub fn generate(generator: &mut Generator, builder: &mut Builder, node: &Box<Node>) {
    let node = cast_node!(NodeWrapper::Program, &node.node);
    match node.opentag.as_str() {
      "<?=" => {
        builder.push(&node.opentag);
        builder.push(" ");
        let expr = generator.generate_nodes_new(
          &node.children,
          &mut GeneratorArgument::for_block()
        );
        builder.push(&expr.to_string(""));
        builder.push(" ?>");
      }
      "<%" => {
        builder.push(&node.opentag);
        generator.generate_nodes(builder, &node.children, &mut GeneratorArgument::for_block());
        builder.new_line();
        builder.push("%>");
      }
      _ => {
        builder.push(&node.opentag);
        generator.generate_nodes(builder, &node.children, &mut GeneratorArgument::for_block());
        builder.new_line();
        builder.push("?>");
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::test_utils::test;

  #[test]
  fn basic() {
    test("<?= \"\"; ?>");
    test("<?php
$a = ++($a++);
?>");
    test("<%
$a = ++($a++);
%>");
  }
}
