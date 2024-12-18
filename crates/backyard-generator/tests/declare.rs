use backyard_generator::generate;
use backyard_parser::arena_parse;

#[test]
fn basic() {
  let arena = bumpalo::Bump::new();
  let asts = arena_parse(&arena, true, "declare(strict_types = 1);").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn with_body() {
  let arena = bumpalo::Bump::new();
  let asts = arena_parse(&arena, true, "declare(ticks = 1, ticks = 1) {\n}").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn with_short_body() {
  let arena = bumpalo::Bump::new();
  let asts = arena_parse(&arena, true, "declare(ticks = 1):\nenddeclare;").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}
