use backyard_nodes::{ cast_node, node::{ Node, NodeType, NodeWrapper } };

use crate::generator::{ Builder, Generator, GeneratorArgument };

pub struct PreGenerator;

impl PreGenerator {
  pub fn generate(generator: &mut Generator, builder: &mut Builder, node: &Box<Node>) {
    let (operator, expr) = match node.node_type {
      NodeType::Variadic => {
        let node = cast_node!(NodeWrapper::Variadic, &node.node);
        ("...", node.statement.to_owned())
      }
      NodeType::Negate => {
        let node = cast_node!(NodeWrapper::Negate, &node.node);
        ("!", Some(node.statement.to_owned()))
      }
      NodeType::Silent => {
        let node = cast_node!(NodeWrapper::Silent, &node.node);
        ("@", Some(node.statement.to_owned()))
      }
      NodeType::Reference => {
        let node = cast_node!(NodeWrapper::Reference, &node.node);
        ("&", Some(node.statement.to_owned()))
      }
      NodeType::Pre => {
        let node = cast_node!(NodeWrapper::Pre, &node.node);
        (node.operator.as_str(), Some(node.statement.to_owned()))
      }
      _ => {
        return;
      }
    };
    builder.push(operator);
    if let Some(expr) = expr {
      generator.generate_node(builder, &expr, &mut GeneratorArgument::default());
    }
  }
}
