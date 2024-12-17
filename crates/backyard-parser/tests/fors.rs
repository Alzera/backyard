use backyard_parser::parse_eval;

#[test]
fn basic() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(&arena, "for ($i = 1; $i <= 10; $i++) {\n}").unwrap();
  insta::assert_yaml_snapshot!(asts.serializable());
}

#[test]
fn short() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(&arena, "for (;;):\nendfor;").unwrap();
  insta::assert_yaml_snapshot!(asts.serializable());
}

#[test]
fn no_body() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(
    &arena,
    "for ($i = 1, $j = 0; $i <= 10; $j += $i, print $i, $i++);"
  ).unwrap();
  insta::assert_yaml_snapshot!(asts.serializable());
}
