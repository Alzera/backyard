use backyard_generator::generate;
use backyard_parser::arena_parse;

#[test]
fn basic() {
  let arena = bumpalo::Bump::new();
  let asts = arena_parse(&arena, true, "$;").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn bracket() {
  let arena = bumpalo::Bump::new();
  let asts = arena_parse(&arena, true, "${$a} = 4;").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn multiple() {
  let arena = bumpalo::Bump::new();
  let asts = arena_parse(&arena, true, "$$a = 4;").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}
