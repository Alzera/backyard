use backyard_generator::generate_serializable_node;
use backyard_parser::parse;

#[test]
fn basic() {
  let asts = parse(true, "class A { use Ale; }").unwrap();
  insta::assert_yaml_snapshot!(generate_serializable_node(&asts).unwrap());
}

#[test]
fn items() {
  let asts = parse(
    true,
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
  insta::assert_yaml_snapshot!(generate_serializable_node(&asts).unwrap());
}
