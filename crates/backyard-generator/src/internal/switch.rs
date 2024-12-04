use backyard_nodes::{ cast_node, node::{ Node, NodeType, NodeWrapper } };

use crate::generator::{ Builder, Generator, GeneratorArgument, DEFAULT_GENERATORS };

use super::block::BlockGenerator;

pub struct SwitchGenerator;

impl SwitchGenerator {
  pub fn generate(generator: &mut Generator, builder: &mut Builder, node: &Box<Node>) {
    let node = cast_node!(NodeWrapper::Switch, &node.node);

    builder.push("switch (");
    generator.generate_node(builder, &node.condition, &mut GeneratorArgument::default());
    builder.push(")");
    let end = if node.is_short { Some("endswitch;") } else { None };
    BlockGenerator::generate_specific(
      generator,
      builder,
      &node.body,
      end,
      &[(NodeType::Case, Self::generate_case)]
    );
  }

  pub fn generate_case(generator: &mut Generator, builder: &mut Builder, node: &Box<Node>) {
    let node = cast_node!(NodeWrapper::Case, &node.node);

    if let Some(n) = &node.condition {
      builder.push("case ");
      generator.generate_node(builder, n, &mut GeneratorArgument::default());
      builder.push(":");
    } else {
      builder.push("default:");
    }
    let mut body = BlockGenerator::generate_base(generator, &node.body, &DEFAULT_GENERATORS);
    body.indent();
    builder.extend(body);
  }
}

#[cfg(test)]
mod tests {
  use crate::test_utils::test_eval;

  #[test]
  fn basic() {
    test_eval("switch ($a):
  case 1:
    echo \"1\";
    break;
endswitch;");
    test_eval(
      "switch ($a) {
  case 1:
    echo \"1\";
    break;
  case 2:
    echo \"2\";
    return;
  default:
    echo \"default\";
}"
    );
  }
}
