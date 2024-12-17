use backyard_parser::parse_eval;

#[test]
fn basic() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(&arena, "class A {\n}").unwrap();
  insta::assert_yaml_snapshot!(asts.serializable());
}

#[test]
fn modifiers() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(&arena, "readonly final class A {\n}").unwrap();
  insta::assert_yaml_snapshot!(asts.serializable());
}

#[test]
fn extended() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(&arena, "class A extends B implements C, D {\n}").unwrap();
  insta::assert_yaml_snapshot!(asts.serializable());
}

#[test]
fn anonymous() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(&arena, "$a = new class {\n};").unwrap();
  insta::assert_yaml_snapshot!(asts.serializable());
}

#[test]
fn parameter() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(&arena, "$a = new class($b) {\n};").unwrap();
  insta::assert_yaml_snapshot!(asts.serializable());
}

#[test]
fn anonymous_extended() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(&arena, "$a = new class($b) extends B implements C, D {\n}").unwrap();
  insta::assert_yaml_snapshot!(asts.serializable());
}
