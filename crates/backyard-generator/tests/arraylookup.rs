use backyard_generator::generate;
use backyard_parser::arena_parse;

#[test]
fn basic() {
  let arena = bumpalo::Bump::new();
  let asts = arena_parse(&arena, true, "array(1, 2)[1];").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn no_expr() {
  let arena = bumpalo::Bump::new();
  let asts = arena_parse(&arena, true, "$a[];").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}
