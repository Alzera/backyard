use backyard_generator::generate;
use backyard_parser::parse_eval;

#[test]
fn basic() {
  let asts = parse_eval(
    "function &a(?int ...$b = 0, String &$c = [0.01, 0x12], bool $d): ?int {\n}"
  ).unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn construct() {
  let asts = parse_eval(
    "class A {
  public function __construct(protected int $x, protected string &...$y = 0) {
  }
}"
  ).unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn arrow() {
  let asts = parse_eval("$a = fn &(int $x): ?int => null;").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn anonymous() {
  let asts = parse_eval("$a = function &(int $x, ?int $y) use ($arg2): static {\n};").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}
