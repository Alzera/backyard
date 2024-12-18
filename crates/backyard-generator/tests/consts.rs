use backyard_generator::generate;
use backyard_parser::arena_parse;

#[test]
fn basic() {
  let arena = bumpalo::Bump::new();
  let asts = arena_parse(&arena, true, "const A = 1, B = 2;").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn with_type() {
  let arena = bumpalo::Bump::new();
  let asts = arena_parse(&arena, true, "class A { const int A = 1; }").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn property() {
  let arena = bumpalo::Bump::new();
  let asts = arena_parse(
    &arena,
    true,
    "class A { public private(set) const A = 1, B = 2; }"
  ).unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}
