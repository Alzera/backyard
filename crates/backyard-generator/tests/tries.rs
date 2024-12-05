use backyard_generator::generate;
use backyard_parser::parse_eval;

#[test]
fn basic() {
  let asts = parse_eval("try {
} catch (Exception $e) {
}").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn multiple_types() {
  let asts = parse_eval("try {
} catch (UnknownGetterException | ReflectionException) {
}").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn finally() {
  let asts = parse_eval(
    "try {
  throw new Error(\"Custom error occurred\");
} catch (FooError $err) {
} catch (Foo2Error | BarError $err) {
} finally {
}"
  ).unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}
