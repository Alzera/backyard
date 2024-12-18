use backyard_generator::generate;
use backyard_parser::arena_parse;

#[test]
fn basic() {
  let arena = bumpalo::Bump::new();
  let asts = arena_parse(&arena, true, "class A { use Ale; }").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn items() {
  let arena = bumpalo::Bump::new();
  let asts = arena_parse(
    &arena,
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
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}
