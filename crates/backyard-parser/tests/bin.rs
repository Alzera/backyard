use backyard_parser::parse_eval;

#[test]
fn basic() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(&arena, "$a = 5 + 5;").unwrap();
  insta::assert_yaml_snapshot!(asts.serializable());
}

#[test]
fn chained() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(
    &arena,
    "$this->a($a)
  ?? $this->b($b) ?? $this->c($c)
    ?? $this->d($d) ?? $this->e($e);"
  ).unwrap();
  insta::assert_yaml_snapshot!(asts.serializable());
}
