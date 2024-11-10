use crate::{
  generator::generator::{ Builder, Generator, GeneratorArgument },
  guard_ok,
  parser::{
    node::{ Node, NodeTraitCast, NodeType },
    nodes::singles::{
      BreakNode,
      CloneNode,
      ContinueNode,
      EchoNode,
      GlobalNode,
      NewNode,
      PrintNode,
      ReturnNode,
      ThrowNode,
    },
  },
};

pub struct SinglesGenerator {}

impl SinglesGenerator {
  pub fn generate(generator: &mut Generator, builder: &mut Builder, node: &Node) {
    let node = match node.get_type() {
      NodeType::Break => {
        builder.push("break");
        guard_ok!(node.to_owned().cast::<BreakNode>(), {
          return;
        }).argument
      }
      NodeType::Continue => {
        builder.push("continue");
        guard_ok!(node.to_owned().cast::<ContinueNode>(), {
          return;
        }).argument
      }
      NodeType::Return => {
        builder.push("return");
        guard_ok!(node.to_owned().cast::<ReturnNode>(), {
          return;
        }).argument
      }
      NodeType::Global => {
        builder.push("global");
        Some(
          guard_ok!(node.to_owned().cast::<GlobalNode>(), {
            return;
          }).argument
        )
      }
      NodeType::Clone => {
        builder.push("clone");
        Some(
          guard_ok!(node.to_owned().cast::<CloneNode>(), {
            return;
          }).argument
        )
      }
      NodeType::Echo => {
        builder.push("echo");
        Some(
          guard_ok!(node.to_owned().cast::<EchoNode>(), {
            return;
          }).argument
        )
      }
      NodeType::New => {
        builder.push("new");
        Some(
          guard_ok!(node.to_owned().cast::<NewNode>(), {
            return;
          }).argument
        )
      }
      NodeType::Print => {
        builder.push("print");
        Some(
          guard_ok!(node.to_owned().cast::<PrintNode>(), {
            return;
          }).argument
        )
      }
      NodeType::Throw => {
        builder.push("throw");
        Some(
          guard_ok!(node.to_owned().cast::<ThrowNode>(), {
            return;
          }).argument
        )
      }
      NodeType::Parent => {
        builder.push("parent");
        None
      }
      NodeType::Static => {
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
