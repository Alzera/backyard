use backyard_generator::generate_serializable_node;
use backyard_parser::parse;

#[test]
fn basic() {
  let asts = parse(true, "self::a;").unwrap();
  insta::assert_yaml_snapshot!(generate_serializable_node(&asts).unwrap());
}

#[test]
fn bracket() {
  let asts = parse(true, "self::{$a};").unwrap();
  insta::assert_yaml_snapshot!(generate_serializable_node(&asts).unwrap());
}

#[test]
fn test_class() {
  let asts = parse(true, "self::class;").unwrap();
  insta::assert_yaml_snapshot!(generate_serializable_node(&asts).unwrap());
}

#[test]
fn variable() {
  let asts = parse(true, "self::$a;").unwrap();
  insta::assert_yaml_snapshot!(generate_serializable_node(&asts).unwrap());
}
