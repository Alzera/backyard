use backyard_parser::parse_eval;

#[test]
fn basic() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(&arena, "#[Attr]
class A {
}").unwrap();
  insta::assert_yaml_snapshot!(asts.serializable());
}

#[test]
fn with_argument() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(&arena, "#[Attr(123)]
class A {
}").unwrap();
  insta::assert_yaml_snapshot!(asts.serializable());
}

#[test]
fn with_named_argument() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(&arena, "#[Attr(a: 123)]
class A {
}").unwrap();
  insta::assert_yaml_snapshot!(asts.serializable());
}

#[test]
fn multiple_items() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(&arena, "#[Attr(123), \\Attr(123)]
class A {
}").unwrap();
  insta::assert_yaml_snapshot!(asts.serializable());
}

#[test]
fn multiple() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(&arena, "#[\\Attr] 
#[Attr(123), \\Attr(123)]
class A {
}").unwrap();
  insta::assert_yaml_snapshot!(asts.serializable());
}
