use backyard_parser::parse_eval;

#[test]
fn basic() {
  let asts = parse_eval("class A { use Ale; }").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn items() {
  let asts = parse_eval(
    "class A {
  use Loggable, Usable {
    log as public;
    log as private alias;
    Loggable::log as aliasLoggable;
    Usable insteadof Loggable;
    Usable::useResource insteadof Loggable;
  }
}"
  ).unwrap();
  insta::assert_yaml_snapshot!(asts);
}
