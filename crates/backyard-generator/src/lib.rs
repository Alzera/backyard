use backyard_nodes::node::Node;
use generator::Generator;

mod generator;
mod internal;
mod test_utils;

pub fn generate(nodes: Vec<Box<Node>>) -> String {
  let mut generator = Generator::new(&nodes);
  generator.start()
}
