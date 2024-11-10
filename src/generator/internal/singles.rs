use crate::{
  generator::generator::{ Builder, Generator, GeneratorArgument },
  guard,
  parser::{
    node::{ Node, NodeTraitCast, NodeType },
    nodes::singles::{
      BreakNode,
      CloneNode,
      ContinueNode,
      EchoNode,
      GlobalNode,
      GotoNode,
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
        guard!(node.to_owned().cast::<BreakNode>()).argument
      }
      NodeType::Continue => {
        builder.push("continue");
        guard!(node.to_owned().cast::<ContinueNode>()).argument
      }
      NodeType::Return => {
        builder.push("return");
        guard!(node.to_owned().cast::<ReturnNode>()).argument
      }
      NodeType::Global => {
        builder.push("global");
        Some(guard!(node.to_owned().cast::<GlobalNode>()).argument)
      }
      NodeType::Clone => {
        builder.push("clone");
        Some(guard!(node.to_owned().cast::<CloneNode>()).argument)
      }
      NodeType::Echo => {
        builder.push("echo");
        Some(guard!(node.to_owned().cast::<EchoNode>()).argument)
      }
      NodeType::New => {
        builder.push("new");
        Some(guard!(node.to_owned().cast::<NewNode>()).argument)
      }
      NodeType::Print => {
        builder.push("print");
        Some(guard!(node.to_owned().cast::<PrintNode>()).argument)
      }
      NodeType::Throw => {
        builder.push("throw");
        Some(guard!(node.to_owned().cast::<ThrowNode>()).argument)
      }
      NodeType::Goto => {
        builder.push("goto");
        Some(guard!(node.to_owned().cast::<GotoNode>()).label)
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

#[cfg(test)]
mod tests {
  use crate::test_utils::test;

  #[test]
  fn basic() {
    test("break;");
    test("break 2;");
    test("continue;");
    test("continue 2;");
    test("return;");
    test("return 2;");
    test("global $a;");
    test("clone $a;");
    test("echo \"Hello\";");
    test("new A;");
    test("print \"Hello\";");
    test("throw new A;");
    test("goto jumpHere;");
    test("parent::a();");
    test("static::a();");
  }
}
