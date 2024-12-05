use backyard_generator::generate;
use backyard_parser::parse_eval;

#[test]
fn basic() {
  let asts = parse_eval("yield;").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn with_argument() {
  let asts = parse_eval("yield $a;").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn with_key() {
  let asts = parse_eval("yield $a => $b;").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn from() {
  let asts = parse_eval("yield from $a;").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}
