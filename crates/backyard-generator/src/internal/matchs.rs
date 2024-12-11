use backyard_nodes::{ cast_node, node::{ Node, NodeType, NodeWrapper } };

use crate::generator::{ Builder, EndMode, Generator, GeneratorArgument, DEFAULT_GENERATORS };

pub struct MatchGenerator;

impl MatchGenerator {
  pub fn generate<'arena>(
    generator: &mut Generator<'arena, '_>,
    builder: &mut Builder,
    node: &Node<'arena>
  ) {
    let node = cast_node!(Match, &node.node);
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
    builder.extend(arms);
    builder.new_line();
    builder.push("}");
  }

  pub fn generate_arm<'arena>(
    generator: &mut Generator<'arena, '_>,
    builder: &mut Builder,
    node: &Node<'arena>
  ) {
    let node = cast_node!(MatchArm, &node.node);
    if !node.conditions.is_empty() {
      let conditions = generator.generate_nodes_new(
        &node.conditions,
        &mut GeneratorArgument::for_parameter(&DEFAULT_GENERATORS)
      );
      builder.push(&conditions.print(" "));
    } else {
      builder.push("default");
    }
    builder.push(" => ");
    generator.generate_node(builder, &node.expr, &mut GeneratorArgument::default());
  }
}
