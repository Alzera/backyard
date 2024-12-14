use backyard_nodes::{ cast_node, Node, NodeType, NodeWrapper };

use crate::generator::{ Builder, Generator, GeneratorArgument };

pub struct StringGenerator;

impl StringGenerator {
  pub fn generate(_: &mut Generator, builder: &mut Builder, node: &Node) {
    let node = cast_node!(String, &node.wrapper);
    builder.push(&format!("{}{}{}", node.quote, node.value, node.quote));
  }

  pub fn generate_encapsed<'arena>(
    generator: &mut Generator<'arena, '_>,
    builder: &mut Builder,
    node: &Node<'arena>
  ) {
    let node = cast_node!(Encapsed, &node.wrapper);
    let quote = format!("{}", node.quote);
    builder.push(&quote);
    let parts = generator
      .generate_nodes_new(
        &node.values,
        &mut GeneratorArgument::generator(&[(NodeType::EncapsedPart, Self::generate_encapsed_part)])
      )
      .print("");
    builder.push(&parts);
    builder.push(&quote);
  }

  pub fn generate_encapsed_part<'arena>(
    generator: &mut Generator<'arena, '_>,
    builder: &mut Builder,
    node: &Node<'arena>
  ) {
    let node = cast_node!(EncapsedPart, &node.wrapper);
    if node.value.node_type == NodeType::String {
      let value = cast_node!(String, &node.value.wrapper);
      builder.push(&value.value.to_string());
      return;
    }
    let expr = generator.generate_node_new(&node.value).print("");
    if node.is_advanced {
      builder.push(format!("{{{}}}", expr).as_str());
    } else {
      builder.push(expr.as_str());
    }
  }

  pub fn generate_nowdoc(_: &mut Generator, builder: &mut Builder, node: &Node) {
    let node = cast_node!(NowDoc, &node.wrapper);
    builder.push(&format!("<<<'{}'", node.label));
    let content = node.value.to_string();
    builder.push(&content);
    if let Some(last) = content.split('\n').last() {
      if !last.chars().all(|x| x.is_whitespace()) {
        builder.push("\n");
      }
    }
    builder.push(&node.label.to_string());
  }

  pub fn generate_heredoc<'arena>(
    generator: &mut Generator<'arena, '_>,
    builder: &mut Builder,
    node: &Node<'arena>
  ) {
    let node = cast_node!(HereDoc, &node.wrapper);
    builder.push(&format!("<<<{}", node.label));
    let parts = generator.generate_nodes_new(
      &node.values,
      &mut GeneratorArgument::generator(&[(NodeType::EncapsedPart, Self::generate_encapsed_part)])
    );
    builder.push(&parts.print(""));
    if let Some(last) = parts.lines.last() {
      if let Some(last) = last.line.split('\n').last() {
        if !last.chars().all(|x| x.is_whitespace()) {
          builder.push("\n");
        }
      }
    }
    builder.push(&node.label.to_string());
  }
}
