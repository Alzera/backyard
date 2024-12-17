use backyard_parser::parse_eval;

#[test]
fn basic() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(&arena, "++$a;").unwrap();
  insta::assert_yaml_snapshot!(asts.serializable());
}

#[test]
fn negate() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(&arena, "!$a;").unwrap();
  insta::assert_yaml_snapshot!(asts.serializable());
}

#[test]
fn silent() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(&arena, "@$a();").unwrap();
  insta::assert_yaml_snapshot!(asts.serializable());
}

#[test]
fn ellipsis() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(&arena, "...$a();").unwrap();
  insta::assert_yaml_snapshot!(asts.serializable());
}

#[test]
fn ambersand() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(&arena, "$a = &$b;").unwrap();
  insta::assert_yaml_snapshot!(asts.serializable());
}
