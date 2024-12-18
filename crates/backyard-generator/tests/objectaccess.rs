use backyard_generator::generate;
use backyard_parser::arena_parse;

#[test]
fn basic() {
  let arena = bumpalo::Bump::new();
  let asts = arena_parse(&arena, true, "$this->from;").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn var() {
  let arena = bumpalo::Bump::new();
  let asts = arena_parse(&arena, true, "$this->$from;").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn bracket() {
  let arena = bumpalo::Bump::new();
  let asts = arena_parse(&arena, true, "$this->{$from};").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn safe() {
  let arena = bumpalo::Bump::new();
  let asts = arena_parse(&arena, true, "$this?->from;").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}
