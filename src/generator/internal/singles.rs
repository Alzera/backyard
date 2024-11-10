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
      NodeType::Goto => {
        builder.push("goto");
        Some(
          guard_ok!(node.to_owned().cast::<GotoNode>(), {
            return;
          }).label
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
