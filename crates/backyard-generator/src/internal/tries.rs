use backyard_nodes::{ cast_node, node::{ Node, NodeType, NodeWrapper } };

use crate::generator::{ Builder, Generator, GeneratorArgument };

use super::block::BlockGenerator;

pub struct TryGenerator;

impl TryGenerator {
  pub fn generate(generator: &mut Generator, builder: &mut Builder, node: &Box<Node>) {
    let node = cast_node!(NodeWrapper::Try, &node.node);
    builder.push("try");
    BlockGenerator::generate(generator, builder, &node.body, None);
    for catch in &node.catches {
      if catch.node_type == NodeType::Finally {
        let node = cast_node!(NodeWrapper::Finally, &catch.node);
        builder.push(" finally");
        BlockGenerator::generate(generator, builder, &node.body, None);
      } else {
        Self::generate_catch(generator, builder, catch);
      }
    }
  }

  pub fn generate_catch(generator: &mut Generator, builder: &mut Builder, node: &Box<Node>) {
    let node = cast_node!(NodeWrapper::Catch, &node.node);
    builder.push(" catch (");
    let types = generator.generate_nodes_new(&node.types, &mut GeneratorArgument::default());
    builder.push(&types.print(" | "));
    if let Some(variable) = &node.variable {
      builder.push(" ");
      generator.generate_node(builder, variable, &mut GeneratorArgument::default());
    }
    builder.push(")");
    BlockGenerator::generate(generator, builder, &node.body, None);
  }
}

#[cfg(test)]
mod tests {
  use crate::test_utils::test_eval;

  #[test]
  fn basic() {
    test_eval(
      "class A {
  public function assertEquals() {
    try {
    } catch (ComparisonFailure $e) {
    }
  }
  protected function toArray() {
  }
}"
    );
    test_eval("try {
} catch (UnknownGetterException | ReflectionException) {
}");
    test_eval(
      "try {
  throw new Error(\"Custom error occurred\");
} catch (FooError $err) {
} catch (Foo2Error | BarError $err) {
} finally {
}"
    );
  }
}
