use backyard_generator::generate;
use backyard_parser::arena_parse;

#[test]
fn basic() {
  let arena = bumpalo::Bump::new();
  let asts = arena_parse(&arena, true, "use const A\\B;").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn alias() {
  let arena = bumpalo::Bump::new();
  let asts = arena_parse(&arena, true, "use const A\\B as A;").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn multiple() {
  let arena = bumpalo::Bump::new();
  let asts = arena_parse(&arena, true, "use A\\B as A, B\\C as B;").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn items() {
  let arena = bumpalo::Bump::new();
  let asts = arena_parse(
    &arena,
    true,
    "use App\\Models\\{
  const User\\UserTesting as UserTestingA,
  User\\UserTestingB as UserTestingB,
  function UserTestingC
};"
  ).unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}
