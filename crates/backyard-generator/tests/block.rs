use backyard_generator::generate;
use backyard_parser::parse_eval;

#[test]
fn basic() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(&arena, "{\n}").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn nested() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(&arena, "{\n\t{\n\t}\n}").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}
