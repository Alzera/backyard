use backyard_generator::generate;
use backyard_parser::parse_eval;

#[test]
fn basic() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(
    &arena,
    "abstract class A {
  public function a() {
  }
  public final static function b();
}"
  ).unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn shuffle_modifier() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(&arena, "abstract class A {
  final static public function b();
}").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}
