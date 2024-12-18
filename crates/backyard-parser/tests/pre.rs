use backyard_parser::parse;

#[test]
fn basic() {
  let asts = parse(true, "++$a;").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn negate() {
  let asts = parse(true, "!$a;").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn silent() {
  let asts = parse(true, "@$a();").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn ellipsis() {
  let asts = parse(true, "...$a();").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn ambersand() {
  let asts = parse(true, "$a = &$b;").unwrap();
  insta::assert_yaml_snapshot!(asts);
}
