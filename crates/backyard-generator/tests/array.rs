use backyard_generator::generate;
use backyard_parser::arena_parse;

#[test]
fn basic() {
  let arena = bumpalo::Bump::new();
  let asts = arena_parse(&arena, true, "array(1, 2);").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn short() {
  let arena = bumpalo::Bump::new();
  let asts = arena_parse(&arena, true, "[1, 2];").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn with_key() {
  let arena = bumpalo::Bump::new();
  let asts = arena_parse(&arena, true, "['a' => 1, 'b' => 2];").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn complex() {
  let arena = bumpalo::Bump::new();
  let asts = arena_parse(
    &arena,
    true,
    "$a = [$key = (is_int($key) ? $value : $key) => $value === true ? $key : $value];"
  ).unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}
