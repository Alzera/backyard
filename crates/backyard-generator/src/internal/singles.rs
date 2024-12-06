use backyard_nodes::{ cast_node, node::{ Node, NodeType, NodeWrapper } };

use crate::generator::{ Builder, Generator, GeneratorArgument };

pub struct SinglesGenerator;

impl SinglesGenerator {
  pub fn generate(generator: &mut Generator, builder: &mut Builder, node: &Box<Node>) {
    let node = match node.node_type {
      NodeType::Break => {
        builder.push("break");
        cast_node!(Break, node.node.to_owned()).statement
      }
      NodeType::Continue => {
        builder.push("continue");
        cast_node!(Continue, node.node.to_owned()).statement
      }
      NodeType::Return => {
        builder.push("return");
        cast_node!(Return, node.node.to_owned()).statement
      }
      NodeType::Clone => {
        builder.push("clone");
        Some(cast_node!(Clone, node.node.to_owned()).statement)
      }
      NodeType::New => {
        builder.push("new");
        Some(cast_node!(New, node.node.to_owned()).statement)
      }
      NodeType::Print => {
        builder.push("print");
        Some(cast_node!(Print, node.node.to_owned()).statement)
      }
      NodeType::Throw => {
        builder.push("throw");
        Some(cast_node!(Throw, node.node.to_owned()).statement)
      }
      NodeType::Goto => {
        builder.push("goto");
        Some(cast_node!(Goto, node.node.to_owned()).label)
      }
      NodeType::Inline => {
        builder.push(" ?>");
        builder.push(&cast_node!(Inline, node.node.to_owned()).text);
        builder.push("<?php ");
        None
      }
      NodeType::Boolean => {
        let node = cast_node!(Boolean, node.node.to_owned());
        if node.is_true {
          builder.push("true");
        } else {
          builder.push("false");
        }
        None
      }
      NodeType::This => {
        builder.push("$this");
        None
      }
      NodeType::Null => {
        builder.push("null");
        None
      }
      NodeType::SelfKeyword => {
        builder.push("self");
        None
      }
      NodeType::Parent => {
        builder.push("parent");
        None
      }
      NodeType::StaticKeyword => {
        builder.push("static");
        None
      }
      _ => {
        return;
      }
    };
    if let Some(node) = node {
      builder.push(" ");
      generator.generate_node(builder, &node, &mut GeneratorArgument::default());
    }
  }
}
