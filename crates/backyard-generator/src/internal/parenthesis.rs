use backyard_nodes::{ cast_node, node::{ Node, NodeWrapper } };

use crate::generator::{ Builder, Generator, GeneratorArgument };

pub struct ParenthesisGenerator;

impl ParenthesisGenerator {
  pub fn generate(generator: &mut Generator, builder: &mut Builder, node: &Box<Node>) {
    let node = cast_node!(NodeWrapper::Parenthesis, &node.node);
    builder.push("(");
    generator.generate_node(builder, &node.statement, &mut GeneratorArgument::default());
    builder.push(")");
  }

  pub fn generate_cast(generator: &mut Generator, builder: &mut Builder, node: &Box<Node>) {
    let node = cast_node!(NodeWrapper::Cast, &node.node);
    builder.push("(");
    builder.push(&node.cast_type);
    builder.push(") ");
    generator.generate_node(builder, &node.expression, &mut GeneratorArgument::default());
  }
}

#[cfg(test)]
mod tests {
  use crate::test_utils::test_eval;

  #[test]
  fn basic() {
    test_eval("$a = (int) $a;");
    test_eval("$a = 5 + 0.5 + (.5 + 0x2e45);");
    test_eval("(fn () => 0)();");
    test_eval(
      "$flags = 
  (RUNKIT_IMPORT_FUNCTIONS | RUNKIT_IMPORT_CLASSES | RUNKIT_IMPORT_CLASS_METHODS
    | RUNKIT_IMPORT_CLASS_CONSTS | RUNKIT_IMPORT_CLASS_PROPS | RUNKIT_IMPORT_OVERRIDE);"
    );
  }
}
