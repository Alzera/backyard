use backyard_generator::generate;
use backyard_parser::arena_parse;

#[test]
fn basic() {
  let arena = bumpalo::Bump::new();
  let asts = arena_parse(&arena, true, "self::a;").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn bracket() {
  let arena = bumpalo::Bump::new();
  let asts = arena_parse(&arena, true, "self::{$a};").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn test_class() {
  let arena = bumpalo::Bump::new();
  let asts = arena_parse(&arena, true, "self::class;").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn variable() {
  let arena = bumpalo::Bump::new();
  let asts = arena_parse(&arena, true, "self::$a;").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}
