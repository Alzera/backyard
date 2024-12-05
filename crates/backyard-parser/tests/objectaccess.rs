use backyard_parser::parse_eval;

#[test]
fn basic() {
  let asts = parse_eval("$this->from;").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn var() {
  let asts = parse_eval("$this->$from;").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn bracket() {
  let asts = parse_eval("$this->{$from};").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn safe() {
  let asts = parse_eval("$this?->from;").unwrap();
  insta::assert_yaml_snapshot!(asts);
}
