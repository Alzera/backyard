use backyard_generator::generate;
use backyard_parser::parse_eval;

#[test]
fn basic() {
  let asts = parse_eval("$this->from;").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn var() {
  let asts = parse_eval("$this->$from;").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn bracket() {
  let asts = parse_eval("$this->{$from};").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn safe() {
  let asts = parse_eval("$this?->from;").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}
