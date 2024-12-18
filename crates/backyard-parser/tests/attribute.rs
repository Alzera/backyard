use backyard_parser::parse;

#[test]
fn basic() {
  let asts = parse(true, "#[Attr]
class A {
}").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn with_argument() {
  let asts = parse(true, "#[Attr(123)]
class A {
}").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn with_named_argument() {
  let asts = parse(true, "#[Attr(a: 123)]
class A {
}").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn multiple_items() {
  let asts = parse(true, "#[Attr(123), \\Attr(123)]
class A {
}").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn multiple() {
  let asts = parse(true, "#[\\Attr] 
#[Attr(123), \\Attr(123)]
class A {
}").unwrap();
  insta::assert_yaml_snapshot!(asts);
}
