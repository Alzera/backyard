use backyard_generator::generate;
use backyard_parser::arena_parse;

#[test]
fn basic() {
  let arena = bumpalo::Bump::new();
  let asts = arena_parse(&arena, true, "$a = 0;").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn decimal() {
  let arena = bumpalo::Bump::new();
  let asts = arena_parse(&arena, true, "$a = 0.1;").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn decimal_only() {
  let arena = bumpalo::Bump::new();
  let asts = arena_parse(&arena, true, "$a = .1;").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn hex() {
  let arena = bumpalo::Bump::new();
  let asts = arena_parse(&arena, true, "$a = 0x0f9bc0;").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn binary() {
  let arena = bumpalo::Bump::new();
  let asts = arena_parse(&arena, true, "$a = 0b1101_1010;").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}
