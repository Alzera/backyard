use backyard_generator::generate;
use backyard_parser::parse_eval;

#[test]
fn basic() {
  let asts = parse_eval(
    "class A {
  protected static var ?A $a = 4;
  public readonly A|callable|null $b = 4;
}"
  ).unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}
