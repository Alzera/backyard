use backyard_generator::generate;
use backyard_parser::arena_parse;

#[test]
fn basic() {
  let arena = bumpalo::Bump::new();
  let asts = arena_parse(&arena, true, "{\n}").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn nested() {
  let arena = bumpalo::Bump::new();
  let asts = arena_parse(&arena, true, "{\n\t{\n\t}\n}").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}
