use backyard_nodes::Node;

use crate::generator::{ Builder, Generator };

pub struct HaltGenerator;

impl HaltGenerator {
  pub fn generate(_: &mut Generator, builder: &mut Builder, _: &Node) {
    builder.push("__halt_compiler();");
  }
}
