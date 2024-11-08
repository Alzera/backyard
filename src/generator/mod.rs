use generator::Generator;

use crate::parser::node::Nodes;

pub mod generator;
pub mod internal;

pub fn generate(nodes: Nodes) -> String {
  let mut generator = Generator::new(nodes);
  let res = generator.start();
  // println!("{:?}", res);
  res
}
