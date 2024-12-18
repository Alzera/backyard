use backyard_generator::generate;
use backyard_parser::arena_parse;

#[test]
fn basic() {
  let arena = bumpalo::Bump::new();
  let asts = arena_parse(&arena, true, "namespace MyApp\\A;").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn with_body() {
  let arena = bumpalo::Bump::new();
  let asts = arena_parse(&arena, true, "namespace MyApp\\B {}").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}
