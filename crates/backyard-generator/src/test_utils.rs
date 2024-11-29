#[allow(dead_code)]
pub fn test(input: &str) {
  let asts = backyard_parser::parse(input);
  assert!(asts.is_ok(), "Error parsing input: {:?}\n{:?}", input, asts);
  // println!("asts: {:?}", asts);
  let gen = crate::generate(asts.unwrap()).unwrap();
  assert_eq!(input, gen);
}

#[allow(dead_code)]
pub fn test_eval(input: &str) {
  let asts = backyard_parser::parse_eval(input);
  assert!(asts.is_ok(), "Error parsing input: {:?}\n{:?}", input, asts);
  // println!("asts: {:?}", asts);
  let gen = crate::generate(asts.unwrap()).unwrap();
  assert_eq!(input, gen);
}
