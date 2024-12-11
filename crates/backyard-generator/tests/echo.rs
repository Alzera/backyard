use backyard_generator::generate;
use backyard_parser::parse_eval;

#[test]
fn basic() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(&arena, "echo \"Hello\";").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn multiple() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(&arena, "echo \"Hello\", \"World\";").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}
