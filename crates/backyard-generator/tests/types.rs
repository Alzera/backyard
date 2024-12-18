use backyard_generator::generate;
use backyard_parser::arena_parse;

#[test]
fn union() {
  let arena = bumpalo::Bump::new();
  let asts = arena_parse(
    &arena,
    true,
    "class A {
  private array|\\Closure $suggestedValues = [];
}"
  ).unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn intersection() {
  let arena = bumpalo::Bump::new();
  let asts = arena_parse(&arena, true, "class A {
    protected \\A&\\B $currentHandler;
}").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn parenthesis() {
  let arena = bumpalo::Bump::new();
  let asts = arena_parse(
    &arena,
    true,
    "class A {
  protected ((\\A|\\C)&\\B)|null $currentHandler2;
}"
  ).unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn break_on_var() {
  let arena = bumpalo::Bump::new();
  let asts = arena_parse(
    &arena,
    true,
    "class A {
  function __construct(protected A&null &$a){
  }
}"
  ).unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn single_parenthesis() {
  let arena = bumpalo::Bump::new();
  let asts = arena_parse(&arena, true, "class A {
  protected (int) $a;
}").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}
