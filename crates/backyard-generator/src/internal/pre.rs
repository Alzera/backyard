use backyard_nodes::{ cast_node, node::{ Node, NodeType, NodeWrapper } };

use crate::generator::{ Builder, Generator, GeneratorArgument };

pub struct PreGenerator {}

impl PreGenerator {
  pub fn generate(generator: &mut Generator, builder: &mut Builder, node: &Box<Node>) {
    let (operator, expr) = match node.node_type {
      NodeType::Negate => {
        let node = cast_node!(NodeWrapper::Negate, &node.node);
        ("!", &node.variable)
      }
      NodeType::Silent => {
        let node = cast_node!(NodeWrapper::Silent, &node.node);
        ("@", &node.variable)
      }
      NodeType::Variadic => {
        let node = cast_node!(NodeWrapper::Variadic, &node.node);
        ("...", &node.expr)
      }
      NodeType::Pre => {
        let node = cast_node!(NodeWrapper::Pre, &node.node);
        (node.operator.as_str(), &node.variable)
      }
      _ => {
        return;
      }
    };
    builder.push(operator);
    generator.generate_node(builder, expr, &mut GeneratorArgument::default());
  }
}

#[cfg(test)]
mod tests {
  use crate::test_utils::test;

  #[test]
  fn basic() {
    test("$a = ++($a++);");
    test("!$a;");
    test("@$a;");
    test("...$a;");
  }
}
