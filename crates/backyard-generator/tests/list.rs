use backyard_generator::generate;
use backyard_parser::parse_eval;

#[test]
fn basic() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(&arena, "list($a, $b) = [0, 1];").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}
