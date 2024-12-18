use backyard_generator::generate_serializable_node;
use backyard_parser::parse;

#[test]
fn basic() {
  let asts = parse(
    true,
    "function &a(?int ...$b = 0, String &$c = [0.01, 0x12], bool $d): ?int {\n}"
  ).unwrap();
  insta::assert_yaml_snapshot!(generate_serializable_node(&asts).unwrap());
}

#[test]
fn construct() {
  let asts = parse(
    true,
    "class A {
  public function __construct(protected int $x, protected string &...$y = 0) {
  }
}"
  ).unwrap();
  insta::assert_yaml_snapshot!(generate_serializable_node(&asts).unwrap());
}

#[test]
fn arrow() {
  let asts = parse(true, "$a = fn &(int $x): ?int => null;").unwrap();
  insta::assert_yaml_snapshot!(generate_serializable_node(&asts).unwrap());
}

#[test]
fn anonymous() {
  let asts = parse(true, "$a = function &(int $x, ?int $y) use ($arg2): static {\n};").unwrap();
  insta::assert_yaml_snapshot!(generate_serializable_node(&asts).unwrap());
}
