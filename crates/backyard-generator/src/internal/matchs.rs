use backyard_nodes::{ cast_node, node::{ Node, NodeType, NodeWrapper } };

use crate::generator::{ Builder, EndMode, Generator, GeneratorArgument, DEFAULT_GENERATORS };

pub struct MatchGenerator {}

impl MatchGenerator {
  pub fn generate(generator: &mut Generator, builder: &mut Builder, node: &Box<Node>) {
    let node = cast_node!(NodeWrapper::Match, &node.node);
    builder.push("match(");
    generator.generate_node(builder, &node.condition, &mut GeneratorArgument::default());
    builder.push(") {");
    let mut arms = generator.generate_nodes_new(
      &node.arms,
      &mut GeneratorArgument::new(
        EndMode::CommaWithoutEnd,
        &[(NodeType::MatchArm, Self::generate_arm)]
      )
    );
    arms.indent();
    builder.extend(&arms);
    builder.new_line();
    builder.push("}");
  }

  pub fn generate_arm(generator: &mut Generator, builder: &mut Builder, node: &Box<Node>) {
    let node = cast_node!(NodeWrapper::MatchArm, &node.node);
    if node.conditions.len() > 0 {
      let conditions = generator.generate_nodes_new(
        &node.conditions,
        &mut GeneratorArgument::for_parameter(&DEFAULT_GENERATORS)
      );
      builder.push(&conditions.to_string(" "));
    } else {
      builder.push("default");
    }
    builder.push(" => ");
    generator.generate_node(builder, &node.body, &mut GeneratorArgument::default());
  }
}

#[cfg(test)]
mod tests {
  use crate::test_utils::test_eval;

  #[test]
  fn basic() {
    test_eval("echo match($myVar) {
  1 => \"One\",
  2, 3 => \"Two\",
  default => \"Other\"
};");
  }
}
