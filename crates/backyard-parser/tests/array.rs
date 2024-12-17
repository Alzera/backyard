use backyard_parser::parse_eval;

#[test]
fn basic() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(&arena, "array(1, 2);").unwrap();
  insta::assert_yaml_snapshot!(asts.serializable());
}

#[test]
fn short() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(&arena, "[1, 2];").unwrap();
  insta::assert_yaml_snapshot!(asts.serializable());
}

#[test]
fn with_key() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(&arena, "['a' => 1, 'b' => 2];").unwrap();
  insta::assert_yaml_snapshot!(asts.serializable());
}

#[test]
fn complex() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(
    &arena,
    "$a = [$key = (is_int($key) ? $value : $key) => $value === true ? $key : $value];"
  ).unwrap();
  insta::assert_yaml_snapshot!(asts.serializable());
}
