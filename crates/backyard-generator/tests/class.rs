use backyard_generator::generate;
use backyard_parser::arena_parse;

#[test]
fn basic() {
  let arena = bumpalo::Bump::new();
  let asts = arena_parse(&arena, true, "class A {\n}").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn modifiers() {
  let arena = bumpalo::Bump::new();
  let asts = arena_parse(&arena, true, "readonly final class A {\n}").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn extended() {
  let arena = bumpalo::Bump::new();
  let asts = arena_parse(&arena, true, "class A extends B implements C, D {\n}").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn anonymous() {
  let arena = bumpalo::Bump::new();
  let asts = arena_parse(&arena, true, "$a = new class {\n};").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn parameter() {
  let arena = bumpalo::Bump::new();
  let asts = arena_parse(&arena, true, "$a = new class($b) {\n};").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn anonymous_extended() {
  let arena = bumpalo::Bump::new();
  let asts = arena_parse(
    &arena,
    true,
    "$a = new class($b) extends B implements C, D {\n}"
  ).unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}
