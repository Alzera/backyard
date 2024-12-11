use backyard_nodes::{ cast_node, node::{ Node, NodeType, NodeWrapper } };

use crate::generator::{ Builder, Generator, GeneratorArgument };

pub struct PreGenerator;

impl PreGenerator {
  pub fn generate<'arena, 'a>(
    generator: &mut Generator<'arena, 'a>,
    builder: &mut Builder,
    node: &Node<'arena>
  ) {
    match node.node_type {
      NodeType::Variadic => {
        let node = cast_node!(Variadic, &node.node);
        builder.push("...");
        if let Some(expr) = &node.statement {
          generator.generate_node(builder, expr, &mut GeneratorArgument::default());
        }
      }
      NodeType::Negate => {
        let node = cast_node!(Negate, &node.node);
        builder.push("!");
        generator.generate_node(builder, &node.statement, &mut GeneratorArgument::default());
      }
      NodeType::Silent => {
        let node = cast_node!(Silent, &node.node);
        builder.push("@");
        generator.generate_node(builder, &node.statement, &mut GeneratorArgument::default());
      }
      NodeType::Reference => {
        let node = cast_node!(Reference, &node.node);
        builder.push("&");
        generator.generate_node(builder, &node.statement, &mut GeneratorArgument::default());
      }
      NodeType::Pre => {
        let node = cast_node!(Pre, &node.node);
        builder.push(&node.operator);
        generator.generate_node(builder, &node.statement, &mut GeneratorArgument::default());
      }
      _ => {
        return;
      }
    };
  }
}
