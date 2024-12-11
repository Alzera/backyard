use backyard_generator::generate;
use backyard_parser::parse_eval;

#[test]
fn basic() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(&arena, "namespace MyApp\\A;").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn with_body() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(&arena, "namespace MyApp\\B {}").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}
