use backyard_generator::generate_serializable_node;
use backyard_parser::parse;

#[test]
fn basic() {
  let asts = parse(true, "interface A {}").unwrap();
  insta::assert_yaml_snapshot!(generate_serializable_node(&asts).unwrap());
}

#[test]
fn without_parenthesis() {
  let asts = parse(
    true,
    "interface A extends B, C {
  const MY_CONSTANT = \"constant value\";
  public function a(int $x, int $y = 0): int;
}"
  ).unwrap();
  insta::assert_yaml_snapshot!(generate_serializable_node(&asts).unwrap());
}
