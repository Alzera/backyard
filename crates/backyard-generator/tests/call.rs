use backyard_generator::generate;
use backyard_parser::parse_eval;

#[test]
fn basic() {
  let asts = parse_eval("call(1, 2);").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn named_argument() {
  let asts = parse_eval("$this(a: 1, b: 2);").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn parenthesis() {
  let asts = parse_eval("(fn() => 5)();").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}
