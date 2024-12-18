use backyard_generator::generate;
use backyard_parser::arena_parse;

#[test]
fn basic() {
  let arena = bumpalo::Bump::new();
  let asts = arena_parse(&arena, true, "$a = 5 * (.5 + 0x2e45);").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn var() {
  let arena = bumpalo::Bump::new();
  let asts = arena_parse(&arena, true, "$a = (int) $a;").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}
