use backyard_nodes::{ cast_node, Node, NodeWrapper };

use crate::generator::{ Builder, Generator };

pub struct CommentGenerator;

impl CommentGenerator {
  pub fn generate(_: &mut Generator, builder: &mut Builder, node: &Node) {
    let node = cast_node!(CommentLine, &node.wrapper);
    builder.push("//");
    builder.push(&node.comment.to_string());
  }

  pub fn generate_block(_: &mut Generator, builder: &mut Builder, node: &Node) {
    let node = cast_node!(CommentBlock, &node.wrapper);
    builder.push("/*");
    builder.new_line();
    let comments = node.comment.to_string();
    let comments: Vec<&str> = comments.split('\n').collect();
    let comments_last_index = comments.clone().len() - 1;
    let comments = comments
      .iter()
      .enumerate()
      .filter_map(|(index, i)| {
        let trimmed = i.trim_start();
        if (index == 0 || index == comments_last_index) && trimmed.is_empty() {
          None
        } else {
          Some(format!(" {}", trimmed))
        }
      });
    for i in comments {
      builder.push(&i);
      builder.new_line();
    }
    builder.push(" */");
  }

  pub fn generate_doc(_: &mut Generator, builder: &mut Builder, node: &Node) {
    let node = cast_node!(CommentDoc, &node.wrapper);
    builder.push("/**");
    builder.new_line();
    let comments = node.comment.to_string();
    let comments: Vec<&str> = comments.split('\n').collect();
    let comments_last_index = comments.clone().len() - 1;
    let comments = comments
      .iter()
      .enumerate()
      .filter_map(|(index, i)| {
        let trimmed = i.trim_start();
        if (index == 0 || index == comments_last_index) && trimmed.is_empty() {
          None
        } else {
          Some(format!(" {}", trimmed))
        }
      });
    for i in comments {
      builder.push(&i);
      builder.new_line();
    }
    builder.push(" */");
  }
}
