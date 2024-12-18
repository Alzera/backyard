use backyard_generator::generate;
use backyard_parser::arena_parse;

#[test]
fn basic() {
  let arena = bumpalo::Bump::new();
  let asts = arena_parse(&arena, true, "$a = 5;").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}
