use backyard_generator::generate_serializable_node;
use backyard_parser::parse;

#[test]
fn basic() {
  let asts = parse(
    true,
    "abstract class A {
  public function a() {
  }
  public final static function b();
}"
  ).unwrap();
  insta::assert_yaml_snapshot!(generate_serializable_node(&asts).unwrap());
}

#[test]
fn shuffle_modifier() {
  let asts = parse(true, "abstract class A {
  final static public function b();
}").unwrap();
  insta::assert_yaml_snapshot!(generate_serializable_node(&asts).unwrap());
}
