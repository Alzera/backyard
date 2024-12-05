use backyard_parser::parse_eval;

#[test]
fn basic() {
  let asts = parse_eval("#[Attr]
class A {
}").unwrap();
  insta::assert_yaml_snapshot!(asts);
}
#[test]
fn with_argument() {
  let asts = parse_eval("#[Attr(123)]
class A {
}").unwrap();
  insta::assert_yaml_snapshot!(asts);
}
#[test]
fn multiple_items() {
  let asts = parse_eval("#[Attr(123), \\Attr(123)]
class A {
}").unwrap();
  insta::assert_yaml_snapshot!(asts);
}
#[test]
fn multiple() {
  let asts = parse_eval("#[\\Attr] 
#[Attr(123), \\Attr(123)]
class A {
}").unwrap();
  insta::assert_yaml_snapshot!(asts);
}
