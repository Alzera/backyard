use backyard_generator::generate_serializable_node;
use backyard_parser::parse;

#[test]
fn basic() {
  let asts = parse(true, "class A {\n}").unwrap();
  insta::assert_yaml_snapshot!(generate_serializable_node(&asts).unwrap());
}

#[test]
fn modifiers() {
  let asts = parse(true, "readonly final class A {\n}").unwrap();
  insta::assert_yaml_snapshot!(generate_serializable_node(&asts).unwrap());
}

#[test]
fn extended() {
  let asts = parse(true, "class A extends B implements C, D {\n}").unwrap();
  insta::assert_yaml_snapshot!(generate_serializable_node(&asts).unwrap());
}

#[test]
fn anonymous() {
  let asts = parse(true, "$a = new class {\n};").unwrap();
  insta::assert_yaml_snapshot!(generate_serializable_node(&asts).unwrap());
}

#[test]
fn parameter() {
  let asts = parse(true, "$a = new class($b) {\n};").unwrap();
  insta::assert_yaml_snapshot!(generate_serializable_node(&asts).unwrap());
}

#[test]
fn anonymous_extended() {
  let asts = parse(true, "$a = new class($b) extends B implements C, D {\n}").unwrap();
  insta::assert_yaml_snapshot!(generate_serializable_node(&asts).unwrap());
}
