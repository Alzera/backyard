#[allow(dead_code)]
pub fn test(input: &str) {
  let asts = backyard_parser::parse(input);
  let gen = crate::generate(asts);
  assert_eq!(input, gen);
}