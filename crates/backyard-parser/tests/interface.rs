use backyard_parser::parse_eval;

#[test]
fn basic() {
  let asts = parse_eval("interface A {}").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn without_parenthesis() {
  let asts = parse_eval(
    "interface A extends B, C {
  const MY_CONSTANT = \"constant value\";
  public function a(int $x, int $y = 0): int;
}"
  ).unwrap();
  insta::assert_yaml_snapshot!(asts);
}
