use backyard_generator::generate;
use backyard_parser::parse_eval;

#[test]
fn basic() {
  let asts = parse_eval("use const A\\B;").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn alias() {
  let asts = parse_eval("use const A\\B as A;").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn multiple() {
  let asts = parse_eval("use A\\B as A, B\\C as B;").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn items() {
  let asts = parse_eval(
    "use App\\Models\\{
  const User\\UserTesting as UserTestingA,
  User\\UserTestingB as UserTestingB,
  function UserTestingC
};"
  ).unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}
