use backyard_generator::generate;
use backyard_parser::arena_parse;

#[test]
fn basic() {
  let arena = bumpalo::Bump::new();
  let asts = arena_parse(&arena, true, "__DIR__ . __FILE__;").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}
