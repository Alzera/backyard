use backyard_nodes::{ cast_node, node::{ Node, NodeType, NodeWrapper } };

use crate::generator::{ Builder, Generator, GeneratorArgument };

pub struct SinglesGenerator {}

impl SinglesGenerator {
  pub fn generate(generator: &mut Generator, builder: &mut Builder, node: &Box<Node>) {
    let node = match node.node_type {
      NodeType::Break => {
        builder.push("break");
        cast_node!(NodeWrapper::Break, node.node.to_owned()).argument
      }
      NodeType::Continue => {
        builder.push("continue");
        cast_node!(NodeWrapper::Continue, node.node.to_owned()).argument
      }
      NodeType::Return => {
        builder.push("return");
        cast_node!(NodeWrapper::Return, node.node.to_owned()).argument
      }
      NodeType::Global => {
        builder.push("global");
        Some(cast_node!(NodeWrapper::Global, node.node.to_owned()).argument)
      }
      NodeType::Clone => {
        builder.push("clone");
        Some(cast_node!(NodeWrapper::Clone, node.node.to_owned()).argument)
      }
      NodeType::Echo => {
        builder.push("echo");
        Some(cast_node!(NodeWrapper::Echo, node.node.to_owned()).argument)
      }
      NodeType::New => {
        builder.push("new");
        Some(cast_node!(NodeWrapper::New, node.node.to_owned()).argument)
      }
      NodeType::Print => {
        builder.push("print");
        Some(cast_node!(NodeWrapper::Print, node.node.to_owned()).argument)
      }
      NodeType::Throw => {
        builder.push("throw");
        Some(cast_node!(NodeWrapper::Throw, node.node.to_owned()).argument)
      }
      NodeType::Goto => {
        builder.push("goto");
        Some(cast_node!(NodeWrapper::Goto, node.node.to_owned()).label)
      }
      NodeType::Boolean => {
        let node = cast_node!(NodeWrapper::Boolean, node.node.to_owned());
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
    test("$this->a();");
    test("parent::a();");
    test("static::a();");
  }
}
