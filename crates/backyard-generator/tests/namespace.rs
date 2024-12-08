use backyard_generator::generate;
use backyard_parser::parse_eval;

#[test]
fn basic() {
  let asts = parse_eval("namespace MyApp\\A;").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn with_body() {
  let asts = parse_eval("namespace MyApp\\B {}").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}
