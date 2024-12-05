use backyard_parser::parse_eval;

#[test]
fn basic() {
  let asts = parse_eval("self::a;").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn bracket() {
  let asts = parse_eval("self::{$a};").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn test_class() {
  let asts = parse_eval("self::class;").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn variable() {
  let asts = parse_eval("self::$a;").unwrap();
  insta::assert_yaml_snapshot!(asts);
}
