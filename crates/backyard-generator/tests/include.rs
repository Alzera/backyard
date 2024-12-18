use backyard_generator::generate;
use backyard_parser::arena_parse;

#[test]
fn basic() {
  let arena = bumpalo::Bump::new();
  let asts = arena_parse(&arena, true, "include(\"a\");").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn without_parenthesis() {
  let arena = bumpalo::Bump::new();
  let asts = arena_parse(&arena, true, "include \"a\";").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn once() {
  let arena = bumpalo::Bump::new();
  let asts = arena_parse(&arena, true, "include_once(\"a\");").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn require() {
  let arena = bumpalo::Bump::new();
  let asts = arena_parse(&arena, true, "require(\"a\");").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn require_once() {
  let arena = bumpalo::Bump::new();
  let asts = arena_parse(&arena, true, "require_once(\"a\");").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}
