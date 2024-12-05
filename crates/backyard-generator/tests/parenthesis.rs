use backyard_generator::generate;
use backyard_parser::parse_eval;

#[test]
fn basic() {
  let asts = parse_eval("$a = 5 * (.5 + 0x2e45);").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn var() {
  let asts = parse_eval("$a = (int) $a;").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}
