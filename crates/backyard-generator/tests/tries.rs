use backyard_generator::generate_serializable_node;
use backyard_parser::parse;

#[test]
fn basic() {
  let asts = parse(true, "try {
} catch (Exception $e) {
}").unwrap();
  insta::assert_yaml_snapshot!(generate_serializable_node(&asts).unwrap());
}

#[test]
fn multiple_types() {
  let asts = parse(true, "try {
} catch (UnknownGetterException | ReflectionException) {
}").unwrap();
  insta::assert_yaml_snapshot!(generate_serializable_node(&asts).unwrap());
}

#[test]
fn finally() {
  let asts = parse(
    true,
    "try {
  throw new Error(\"Custom error occurred\");
} catch (FooError $err) {
} catch (Foo2Error | BarError $err) {
} finally {
}"
  ).unwrap();
  insta::assert_yaml_snapshot!(generate_serializable_node(&asts).unwrap());
}
