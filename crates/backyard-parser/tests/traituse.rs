use backyard_parser::parse;

#[test]
fn basic() {
  let asts = parse(true, "class A { use Ale; }").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn items() {
  let asts = parse(
    true,
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
  insta::assert_yaml_snapshot!(asts);
}
