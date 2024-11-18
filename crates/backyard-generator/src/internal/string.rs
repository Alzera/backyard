use backyard_nodes::{ cast_node, node::{ Node, NodeType, NodeWrapper } };

use crate::generator::{ Builder, Generator, GeneratorArgument };

pub struct StringGenerator {}

impl StringGenerator {
  pub fn generate(_: &mut Generator, builder: &mut Builder, node: &Box<Node>) {
    let node = cast_node!(NodeWrapper::String, &node.node);
    builder.push(format!("\"{}\"", node.value).as_str());
  }

  pub fn generate_encapsed(generator: &mut Generator, builder: &mut Builder, node: &Box<Node>) {
    let node = cast_node!(NodeWrapper::Encapsed, &node.node);
    let parts = generator.generate_nodes_new(
      &node.values,
      &mut GeneratorArgument::generator(&[(NodeType::EncapsedPart, Self::generate_encapsed_part)])
    );
    builder.push(format!("\"{}\"", parts.to_string("")).as_str());
  }

  pub fn generate_encapsed_part(
    generator: &mut Generator,
    builder: &mut Builder,
    node: &Box<Node>
  ) {
    let node = cast_node!(NodeWrapper::EncapsedPart, &node.node);
    if node.value.node_type == NodeType::String {
      let value = cast_node!(NodeWrapper::String, &node.value.node);
      builder.push(&value.value);
      return;
    }
    let expr = generator.generate_node_new(&node.value).to_string("");
    if node.is_advanced {
      builder.push(format!("{{{}}}", expr).as_str());
    } else {
      builder.push(expr.as_str());
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::test_utils::test;

  #[test]
  fn basic() {
    test("$a = \"ale\" . \" ini string $ \\\" \\$var $b {${\"ale\" . 5}} {$a}\";");
  }
}
