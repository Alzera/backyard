use backyard_nodes::{ cast_node, Node, NodeType, NodeWrapper };

use crate::generator::{ Builder, Generator, GeneratorArgument };

pub struct SinglesGenerator;

impl SinglesGenerator {
  pub fn generate<'arena>(
    generator: &mut Generator<'arena, '_>,
    builder: &mut Builder,
    node: &Node<'arena>
  ) {
    match node.node_type {
      NodeType::Break => {
        builder.push("break");
        if let Some(node) = &cast_node!(Break, &node.wrapper).statement {
          builder.push(" ");
          generator.generate_node(builder, node, &mut GeneratorArgument::default());
        }
      }
      NodeType::Continue => {
        builder.push("continue");
        if let Some(node) = &cast_node!(Continue, &node.wrapper).statement {
          builder.push(" ");
          generator.generate_node(builder, node, &mut GeneratorArgument::default());
        }
      }
      NodeType::Return => {
        builder.push("return");
        if let Some(node) = &cast_node!(Return, &node.wrapper).statement {
          builder.push(" ");
          generator.generate_node(builder, node, &mut GeneratorArgument::default());
        }
      }
      NodeType::Clone => {
        builder.push("clone ");
        generator.generate_node(
          builder,
          &cast_node!(Clone, &node.wrapper).statement,
          &mut GeneratorArgument::default()
        );
      }
      NodeType::New => {
        builder.push("new ");
        generator.generate_node(
          builder,
          &cast_node!(New, &node.wrapper).statement,
          &mut GeneratorArgument::default()
        );
      }
      NodeType::Print => {
        builder.push("print ");
        generator.generate_node(
          builder,
          &cast_node!(Print, &node.wrapper).statement,
          &mut GeneratorArgument::default()
        );
      }
      NodeType::Throw => {
        builder.push("throw ");
        generator.generate_node(
          builder,
          &cast_node!(Throw, &node.wrapper).statement,
          &mut GeneratorArgument::default()
        );
      }
      NodeType::Goto => {
        builder.push("goto ");
        generator.generate_node(
          builder,
          &cast_node!(Goto, &node.wrapper).label,
          &mut GeneratorArgument::default()
        );
      }
      NodeType::Inline => {
        builder.push(" ?>");
        builder.push(&cast_node!(Inline, &node.wrapper).text);
        builder.push("<?php ");
      }
      NodeType::Boolean => {
        let node = cast_node!(Boolean, &node.wrapper);
        if node.is_true {
          builder.push("true");
        } else {
          builder.push("false");
        }
      }
      NodeType::This => {
        builder.push("$this");
      }
      NodeType::Null => {
        builder.push("null");
      }
      NodeType::SelfKeyword => {
        builder.push("self");
      }
      NodeType::Parent => {
        builder.push("parent");
      }
      NodeType::StaticKeyword => {
        builder.push("static");
      }
      _ => {}
    }
  }
}
