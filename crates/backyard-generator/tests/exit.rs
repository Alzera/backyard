use backyard_generator::generate;
use backyard_parser::arena_parse;

#[test]
fn basic() {
  let arena = bumpalo::Bump::new();
  let asts = arena_parse(&arena, true, "exit;").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn with_argument() {
  let arena = bumpalo::Bump::new();
  let asts = arena_parse(&arena, true, "exit(0);").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}
