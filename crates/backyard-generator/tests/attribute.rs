use backyard_generator::generate;
use backyard_parser::arena_parse;

#[test]
fn basic() {
  let arena = bumpalo::Bump::new();
  let asts = arena_parse(&arena, true, "#[Attr]
class A {
}").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn with_argument() {
  let arena = bumpalo::Bump::new();
  let asts = arena_parse(&arena, true, "#[Attr(123)]
class A {
}").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn with_named_argument() {
  let arena = bumpalo::Bump::new();
  let asts = arena_parse(&arena, true, "#[Attr(a: 123)]
class A {
}").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn multiple_items() {
  let arena = bumpalo::Bump::new();
  let asts = arena_parse(&arena, true, "#[Attr(123), \\Attr(123)]
class A {
}").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn multiple() {
  let arena = bumpalo::Bump::new();
  let asts = arena_parse(&arena, true, "#[\\Attr] 
#[Attr(123), \\Attr(123)]
class A {
}").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}
