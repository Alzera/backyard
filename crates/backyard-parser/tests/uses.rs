use backyard_parser::parse;

#[test]
fn basic() {
  let asts = parse(true, "use const A\\B;").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn alias() {
  let asts = parse(true, "use const A\\B as A;").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn multiple() {
  let asts = parse(true, "use A\\B as A, B\\C as B;").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn items() {
  let asts = parse(
    true,
    "use App\\Models\\{
  const User\\UserTesting as UserTestingA,
  User\\UserTestingB as UserTestingB,
  function UserTestingC
};"
  ).unwrap();
  insta::assert_yaml_snapshot!(asts);
}
