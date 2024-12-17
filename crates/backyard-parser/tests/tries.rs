use backyard_parser::parse_eval;

#[test]
fn basic() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(&arena, "try {
} catch (Exception $e) {
}").unwrap();
  insta::assert_yaml_snapshot!(asts.serializable());
}

#[test]
fn multiple_types() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(
    &arena,
    "try {
} catch (UnknownGetterException | ReflectionException) {
}"
  ).unwrap();
  insta::assert_yaml_snapshot!(asts.serializable());
}

#[test]
fn finally() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(
    &arena,
    "try {
  throw new Error(\"Custom error occurred\");
} catch (FooError $err) {
} catch (Foo2Error | BarError $err) {
} finally {
}"
  ).unwrap();
  insta::assert_yaml_snapshot!(asts.serializable());
}
