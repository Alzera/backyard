use backyard_parser::parse;

#[test]
fn basic() {
  let asts = parse(true, "include(\"a\");").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn without_parenthesis() {
  let asts = parse(true, "include \"a\";").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn once() {
  let asts = parse(true, "include_once(\"a\");").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn require() {
  let asts = parse(true, "require(\"a\");").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn require_once() {
  let asts = parse(true, "require_once(\"a\");").unwrap();
  insta::assert_yaml_snapshot!(asts);
}
