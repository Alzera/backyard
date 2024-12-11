use backyard_generator::generate;
use backyard_parser::parse_eval;

#[test]
fn basic() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(&arena, "$this->from;").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn var() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(&arena, "$this->$from;").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn bracket() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(&arena, "$this->{$from};").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn safe() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(&arena, "$this?->from;").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}
