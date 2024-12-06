use backyard_nodes::{ cast_node, node::{ Node, NodeType, NodeWrapper } };

use crate::generator::{ Builder, Generator };

pub struct TypeGenerator;

impl TypeGenerator {
  pub fn generate(generator: &mut Generator, builder: &mut Builder, node: &Node) {
    match node.node_type {
      NodeType::Type => Self::generate_basic(generator, builder, node),
      NodeType::UnionType => Self::generate_union(generator, builder, node),
      NodeType::IntersectionType => Self::generate_intersection(generator, builder, node),
      _ => panic!("TypeGenerator::generate: failed to generate type"),
    }
  }

  fn generate_basic(_: &mut Generator, builder: &mut Builder, node: &Node) {
    let node = cast_node!(Type, &node.node);
    if node.is_nullable {
      builder.push("?");
    }
    builder.push(&node.name);
  }

  fn generate_union(generator: &mut Generator, builder: &mut Builder, node: &Node) {
    let node = cast_node!(UnionType, &node.node);
    let types = Self::map_types(generator, &node.types);
    builder.push(&types.join("|"));
  }

  fn generate_intersection(generator: &mut Generator, builder: &mut Builder, node: &Node) {
    let node = cast_node!(IntersectionType, &node.node);
    let types = Self::map_types(generator, &node.types);

    builder.push(&types.join("&"));
  }

  fn map_types(generator: &mut Generator, types: &[Box<Node>]) -> Vec<String> {
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
