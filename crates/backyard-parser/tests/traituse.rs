use backyard_parser::parse_eval;

#[test]
fn basic() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(&arena, "class A { use Ale; }").unwrap();
  insta::assert_yaml_snapshot!(asts.serializable());
}

#[test]
fn items() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(
    &arena,
    "class A {
  use Loggable, Usable {
    log as public;
    get as private alias;
    Loggable::log as aliasLoggable;
    Usable insteadof Loggable;
    Usable::useResource insteadof Loggable;
  }
}"
  ).unwrap();
  insta::assert_yaml_snapshot!(asts.serializable());
}
