use backyard_parser::parse_eval;

#[test]
fn basic() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(&arena, "do {\n} while (false);").unwrap();
  insta::assert_yaml_snapshot!(asts.serializable());
}

#[test]
fn separated_by_comment() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(&arena, "do {\n}\n// this comment\nwhile (false);").unwrap();
  insta::assert_yaml_snapshot!(asts.serializable());
}
