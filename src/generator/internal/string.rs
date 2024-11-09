use crate::{
  generator::generator::{ Builder, Generator, GeneratorArgument },
  guard_ok,
  parser::{
    node::{ Node, NodeTraitCast, NodeType },
    nodes::string::{ EncapsedNode, EncapsedPartNode, StringNode },
  },
};

pub struct StringGenerator {}

impl StringGenerator {
  pub fn generate(_: &mut Generator, builder: &mut Builder, node: &Node) {
    let node = guard_ok!(node.to_owned().cast::<StringNode>(), {
      return;
    });
    builder.push(format!("\"{}\"", node.value).as_str());
  }

  pub fn generate_encapsed(generator: &mut Generator, builder: &mut Builder, node: &Node) {
    let node = guard_ok!(node.to_owned().cast::<EncapsedNode>(), {
      return;
    });
    let parts = generator.generate_nodes_new(
      &node.values,
      &mut GeneratorArgument::generator(&[(NodeType::EncapsedPart, Self::generate_encapsed_part)])
    );
    builder.push(format!("\"{}\"", parts.to_string("")).as_str());
  }

  pub fn generate_encapsed_part(generator: &mut Generator, builder: &mut Builder, node: &Node) {
    let node = guard_ok!(node.to_owned().cast::<EncapsedPartNode>(), {
      return;
    });
    if node.value.get_type() == NodeType::String {
      let value = guard_ok!(node.value.to_owned().cast::<StringNode>(), {
        return;
      });
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
