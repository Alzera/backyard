use backyard_parser::parse;

#[test]
fn basic() {
  let asts = parse(true, "$this->from;").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn var() {
  let asts = parse(true, "$this->$from;").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn bracket() {
  let asts = parse(true, "$this->{$from};").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn safe() {
  let asts = parse(true, "$this?->from;").unwrap();
  insta::assert_yaml_snapshot!(asts);
}
