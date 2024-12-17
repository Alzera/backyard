use backyard_parser::parse_eval;

#[test]
fn basic() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(&arena, "interface A {}").unwrap();
  insta::assert_yaml_snapshot!(asts.serializable());
}

#[test]
fn without_parenthesis() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(
    &arena,
    "interface A extends B, C {
  const MY_CONSTANT = \"constant value\";
  public function a(int $x, int $y = 0): int;
}"
  ).unwrap();
  insta::assert_yaml_snapshot!(asts.serializable());
}
