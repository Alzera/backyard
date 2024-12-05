use backyard_generator::generate;
use backyard_parser::parse_eval;

#[test]
fn basic() {
  let asts = parse_eval("const A = 1, B = 2;").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn property() {
  let asts = parse_eval("class A { public const A = 1, B = 2; }").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}
