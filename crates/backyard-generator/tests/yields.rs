use backyard_generator::generate;
use backyard_parser::parse_eval;

#[test]
fn basic() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(&arena, "yield;").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn with_argument() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(&arena, "yield $a;").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn with_key() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(&arena, "yield $a => $b;").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn from() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(&arena, "yield from $a;").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}
