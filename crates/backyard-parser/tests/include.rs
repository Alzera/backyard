use backyard_parser::parse_eval;

#[test]
fn basic() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(&arena, "include(\"a\");").unwrap();
  insta::assert_yaml_snapshot!(asts.serializable());
}

#[test]
fn without_parenthesis() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(&arena, "include \"a\";").unwrap();
  insta::assert_yaml_snapshot!(asts.serializable());
}

#[test]
fn once() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(&arena, "include_once(\"a\");").unwrap();
  insta::assert_yaml_snapshot!(asts.serializable());
}

#[test]
fn require() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(&arena, "require(\"a\");").unwrap();
  insta::assert_yaml_snapshot!(asts.serializable());
}

#[test]
fn require_once() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(&arena, "require_once(\"a\");").unwrap();
  insta::assert_yaml_snapshot!(asts.serializable());
}
