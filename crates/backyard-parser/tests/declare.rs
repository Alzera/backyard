use backyard_parser::parse_eval;

#[test]
fn basic() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(&arena, "declare(strict_types = 1);").unwrap();
  insta::assert_yaml_snapshot!(asts.serializable());
}

#[test]
fn with_body() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(&arena, "declare(ticks = 1, ticks = 1) {\n}").unwrap();
  insta::assert_yaml_snapshot!(asts.serializable());
}

#[test]
fn with_short_body() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(&arena, "declare(ticks = 1):\nenddeclare;").unwrap();
  insta::assert_yaml_snapshot!(asts.serializable());
}
