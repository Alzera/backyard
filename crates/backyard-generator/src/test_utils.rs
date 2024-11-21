#[allow(dead_code)]
pub fn test(input: &str) {
  let asts = backyard_parser::parse(input);
  assert!(asts.is_ok(), "Error parsing input: {:?}\n{:?}", input, asts);
  let gen = crate::generate(asts.unwrap());
  assert_eq!(input, gen);
}
