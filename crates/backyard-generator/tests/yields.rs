use backyard_generator::generate;
use backyard_parser::arena_parse;

#[test]
fn basic() {
  let arena = bumpalo::Bump::new();
  let asts = arena_parse(&arena, true, "yield;").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn with_argument() {
  let arena = bumpalo::Bump::new();
  let asts = arena_parse(&arena, true, "yield $a;").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn with_key() {
  let arena = bumpalo::Bump::new();
  let asts = arena_parse(&arena, true, "yield $a => $b;").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn from() {
  let arena = bumpalo::Bump::new();
  let asts = arena_parse(&arena, true, "yield from $a;").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}
