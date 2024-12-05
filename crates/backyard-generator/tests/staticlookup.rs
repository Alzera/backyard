use backyard_generator::generate;
use backyard_parser::parse_eval;

#[test]
fn basic() {
  let asts = parse_eval("self::a;").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn bracket() {
  let asts = parse_eval("self::{$a};").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn test_class() {
  let asts = parse_eval("self::class;").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn variable() {
  let asts = parse_eval("self::$a;").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}
