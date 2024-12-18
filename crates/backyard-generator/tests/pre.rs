use backyard_generator::generate;
use backyard_parser::arena_parse;

#[test]
fn basic() {
  let arena = bumpalo::Bump::new();
  let asts = arena_parse(&arena, true, "++$a;").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn negate() {
  let arena = bumpalo::Bump::new();
  let asts = arena_parse(&arena, true, "!$a;").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn silent() {
  let arena = bumpalo::Bump::new();
  let asts = arena_parse(&arena, true, "@$a();").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn ellipsis() {
  let arena = bumpalo::Bump::new();
  let asts = arena_parse(&arena, true, "...$a();").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn ambersand() {
  let arena = bumpalo::Bump::new();
  let asts = arena_parse(&arena, true, "$a = &$b;").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}
