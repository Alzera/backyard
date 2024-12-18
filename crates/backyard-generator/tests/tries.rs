use backyard_generator::generate;
use backyard_parser::arena_parse;

#[test]
fn basic() {
  let arena = bumpalo::Bump::new();
  let asts = arena_parse(&arena, true, "try {
} catch (Exception $e) {
}").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn multiple_types() {
  let arena = bumpalo::Bump::new();
  let asts = arena_parse(
    &arena,
    true,
    "try {
} catch (UnknownGetterException | ReflectionException) {
}"
  ).unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn finally() {
  let arena = bumpalo::Bump::new();
  let asts = arena_parse(
    &arena,
    true,
    "try {
  throw new Error(\"Custom error occurred\");
} catch (FooError $err) {
} catch (Foo2Error | BarError $err) {
} finally {
}"
  ).unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}
