use backyard_parser::parse_eval;

#[test]
fn basic() {
  let asts = parse_eval("class A {\n}").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn modifiers() {
  let asts = parse_eval("readonly final class A {\n}").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn extended() {
  let asts = parse_eval("class A extends B implements C, D {\n}").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn anonymous() {
  let asts = parse_eval("$a = new class {\n};").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn parameter() {
  let asts = parse_eval("$a = new class($b) {\n};").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn anonymous_extended() {
  let asts = parse_eval("$a = new class($b) extends B implements C, D {\n}").unwrap();
  insta::assert_yaml_snapshot!(asts);
}
