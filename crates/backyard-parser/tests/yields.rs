use backyard_parser::parse;

#[test]
fn basic() {
  let asts = parse(true, "yield;").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn with_argument() {
  let asts = parse(true, "yield $a;").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn with_key() {
  let asts = parse(true, "yield $a => $b;").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn from() {
  let asts = parse(true, "yield from $a;").unwrap();
  insta::assert_yaml_snapshot!(asts);
}
