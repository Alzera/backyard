use crate::{ generator::generate, parser::parse };

#[allow(dead_code)]
pub fn test(input: &str) {
  let asts = parse(input);
  let gen = generate(asts);
  assert_eq!(input, gen);
}
