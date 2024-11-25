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
      NodeType::Inline => {
        builder.push(&cast_node!(NodeWrapper::Inline, node.node.to_owned()).text);
        None
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

#[cfg(test)]
mod tests {
  use crate::test_utils::{ test, test_eval };

  #[test]
  fn basic() {
    test_eval("break;");
    test_eval("break 2;");
    test_eval("continue;");
    test_eval("continue 2;");
    test_eval("return;");
    test_eval("return 2;");
    test_eval("global $a;");
    test_eval("clone $a;");
    test_eval("echo \"Hello\";");
    test_eval("new A;");
    test_eval("print \"Hello\";");
    test_eval("throw new A;");
    test_eval("goto jumpHere;");
    test_eval("$this->a();");
    test_eval("parent::a();");
    test_eval("static::a();");
    test("<div>this is an inline</div>");
    test("<div><?php
echo \"Hello\";
?> world<?= \"!\"; ?></div>");
  }
}
