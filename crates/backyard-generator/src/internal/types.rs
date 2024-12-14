use backyard_nodes::{ cast_node, Node, NodeType, NodeWrapper };

use crate::generator::{ Builder, Generator };

pub struct TypeGenerator;

impl TypeGenerator {
  pub fn generate<'arena>(
    generator: &mut Generator<'arena, '_>,
    builder: &mut Builder,
    node: &Node<'arena>
  ) {
    match node.node_type {
      NodeType::Type => Self::generate_basic(generator, builder, node),
      NodeType::UnionType => Self::generate_union(generator, builder, node),
      NodeType::IntersectionType => Self::generate_intersection(generator, builder, node),
      _ => panic!("TypeGenerator::generate: failed to generate type"),
    }
  }

  fn generate_basic(_: &mut Generator, builder: &mut Builder, node: &Node) {
    let node = cast_node!(Type, &node.wrapper);
    if node.is_nullable {
      builder.push("?");
    }
    builder.push(&node.name.to_string());
  }

  fn generate_union<'arena>(
    generator: &mut Generator<'arena, '_>,
    builder: &mut Builder,
    node: &Node<'arena>
  ) {
    let node = cast_node!(UnionType, &node.wrapper);
    let types = Self::map_types(generator, &node.types);
    builder.push(&types.join("|"));
  }

  fn generate_intersection<'arena>(
    generator: &mut Generator<'arena, '_>,
    builder: &mut Builder,
    node: &Node<'arena>
  ) {
    let node = cast_node!(IntersectionType, &node.wrapper);
    let types = Self::map_types(generator, &node.types);

    builder.push(&types.join("&"));
  }

  fn map_types<'arena>(
    generator: &mut Generator<'arena, '_>,
    types: &[Node<'arena>]
  ) -> Vec<String> {
    types
      .iter()
      .map(|x| {
        let mut scoped_builder = Builder::new();
        scoped_builder.new_line();
        match x.node_type {
          NodeType::Type => Self::generate_basic(generator, &mut scoped_builder, x),
          NodeType::UnionType => {
            scoped_builder.push("(");
            Self::generate_union(generator, &mut scoped_builder, x);
            scoped_builder.push(")");
          }
          NodeType::IntersectionType => {
            scoped_builder.push("(");
            Self::generate_intersection(generator, &mut scoped_builder, x);
            scoped_builder.push(")");
          }
          _ => {
            panic!("TypeGenerator::generate_union: failed to get type");
          }
        }
        scoped_builder.print("")
      })
      .collect::<Vec<String>>()
  }
}
