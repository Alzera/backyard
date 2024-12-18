use backyard_generator::generate;
use backyard_parser::arena_parse;

#[test]
fn basic() {
  let arena = bumpalo::Bump::new();
  let asts = arena_parse(&arena, true, "echo \"Hello\";").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn multiple() {
  let arena = bumpalo::Bump::new();
  let asts = arena_parse(&arena, true, "echo \"Hello\", \"World\";").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}
