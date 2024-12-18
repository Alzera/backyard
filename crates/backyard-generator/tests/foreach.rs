use backyard_generator::generate;
use backyard_parser::arena_parse;

#[test]
fn basic() {
  let arena = bumpalo::Bump::new();
  let asts = arena_parse(&arena, true, "foreach ($a as &$b) {\n}").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn with_key() {
  let arena = bumpalo::Bump::new();
  let asts = arena_parse(&arena, true, "foreach ($a as $b => $c) {\n}").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn no_block() {
  let arena = bumpalo::Bump::new();
  let asts = arena_parse(&arena, true, "foreach ($a as $b => $c)\n\t$d = 5;").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn short_block() {
  let arena = bumpalo::Bump::new();
  let asts = arena_parse(&arena, true, "foreach ($a as $b => $c):\n\tendforeach;").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}
