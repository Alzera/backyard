use backyard_parser::parse_eval;

#[test]
fn basic() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(&arena, "use const A\\B;").unwrap();
  insta::assert_yaml_snapshot!(asts.serializable());
}

#[test]
fn alias() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(&arena, "use const A\\B as A;").unwrap();
  insta::assert_yaml_snapshot!(asts.serializable());
}

#[test]
fn multiple() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(&arena, "use A\\B as A, B\\C as B;").unwrap();
  insta::assert_yaml_snapshot!(asts.serializable());
}

#[test]
fn items() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(
    &arena,
    "use App\\Models\\{
  const User\\UserTesting as UserTestingA,
  User\\UserTestingB as UserTestingB,
  function UserTestingC
};"
  ).unwrap();
  insta::assert_yaml_snapshot!(asts.serializable());
}
