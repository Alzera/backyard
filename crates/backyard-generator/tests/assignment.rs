use backyard_generator::generate;
use backyard_parser::parse_eval;

#[test]
fn basic() {
  let asts = parse_eval("$a = 5;").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}
