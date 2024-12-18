use backyard_parser::parse;

#[test]
fn basic() {
  let asts = parse(true, "self::a;").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn bracket() {
  let asts = parse(true, "self::{$a};").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn test_class() {
  let asts = parse(true, "self::class;").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn variable() {
  let asts = parse(true, "self::$a;").unwrap();
  insta::assert_yaml_snapshot!(asts);
}
