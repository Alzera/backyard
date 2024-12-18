use backyard_generator::generate;
use backyard_parser::arena_parse;

#[test]
fn basic() {
  let arena = bumpalo::Bump::new();
  let asts = arena_parse(&arena, true, "call(1, 2);").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn named_argument() {
  let arena = bumpalo::Bump::new();
  let asts = arena_parse(&arena, true, "$this(a: 1, b: 2);").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn parenthesis() {
  let arena = bumpalo::Bump::new();
  let asts = arena_parse(&arena, true, "(fn() => 5)();").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}
