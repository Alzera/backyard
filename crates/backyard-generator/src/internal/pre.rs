use backyard_nodes::{ cast_node, Node, NodeType, NodeWrapper };

use crate::generator::{ Builder, Generator, GeneratorArgument };

pub struct PreGenerator;

impl PreGenerator {
  pub fn generate<'arena>(
    generator: &mut Generator<'arena, '_>,
    builder: &mut Builder,
    node: &Node<'arena>
  ) {
    match node.node_type {
      NodeType::Variadic => {
        let node = cast_node!(Variadic, &node.wrapper);
        builder.push("...");
        if let Some(expr) = &node.statement {
          generator.generate_node(builder, expr, &mut GeneratorArgument::default());
        }
      }
      NodeType::Negate => {
        let node = cast_node!(Negate, &node.wrapper);
        builder.push("!");
        generator.generate_node(builder, &node.statement, &mut GeneratorArgument::default());
      }
      NodeType::Silent => {
        let node = cast_node!(Silent, &node.wrapper);
        builder.push("@");
        generator.generate_node(builder, &node.statement, &mut GeneratorArgument::default());
      }
      NodeType::Reference => {
        let node = cast_node!(Reference, &node.wrapper);
        builder.push("&");
        generator.generate_node(builder, &node.statement, &mut GeneratorArgument::default());
      }
      NodeType::Pre => {
        let node = cast_node!(Pre, &node.wrapper);
        builder.push(&node.operator.to_string());
        generator.generate_node(builder, &node.statement, &mut GeneratorArgument::default());
      }
      _ => {}
    }
  }
}
